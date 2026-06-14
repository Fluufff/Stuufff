CREATE OR REPLACE FUNCTION create_audit_for_table(target_table regclass)
RETURNS void AS $$
DECLARE
    table_schema text;
    table_name text;
    audit_table_name text;
    audit_trigger_function_name text;
    audit_trigger_name text;
    create_audit_table_sql text;
    create_trigger_function_sql text;
    create_trigger_sql text;
BEGIN
    -- Get schema and table name
    SELECT nspname, relname
    INTO table_schema, table_name
    FROM pg_class c
    JOIN pg_namespace n ON n.oid = c.relnamespace
    WHERE c.oid = target_table;

    audit_table_name := format('%I.audit_%I', table_schema, table_name);
    audit_trigger_function_name := format('%I.audit_trigger_%I', table_schema, table_name);
    audit_trigger_name := format('audit_trigger_%I', table_name);

    -- Step 1: Create the audit table
    create_audit_table_sql := format($sql$
        CREATE TABLE IF NOT EXISTS %s (
            audit_id bigserial PRIMARY KEY,
            operation text NOT NULL,
            changed_at timestamptz DEFAULT now(),
            changed_by text DEFAULT current_user,
            old_data jsonb,
            new_data jsonb
        );
    $sql$, audit_table_name);
    EXECUTE create_audit_table_sql;

    -- Step 2: Create the trigger function in the same schema as the table
    create_trigger_function_sql := format($sql$
        CREATE OR REPLACE FUNCTION %s()
        RETURNS trigger AS $_$
        DECLARE
            oldjson jsonb;
            newjson jsonb;
        BEGIN
            oldjson := to_jsonb(OLD);
            newjson := to_jsonb(NEW);
            IF (TG_OP = 'INSERT') THEN
                INSERT INTO %s(operation, new_data)
                VALUES ('INSERT', newjson);
                RETURN NEW;
            ELSIF (TG_OP = 'UPDATE') THEN
                DECLARE
                    diff jsonb := '{}';
                    key text;
                BEGIN
                    FOR key IN SELECT jsonb_object_keys(newjson) LOOP
                        IF newjson->key IS DISTINCT FROM oldjson->key THEN
                            diff := diff || jsonb_build_object(key, jsonb_build_array(oldjson->key, newjson->key));
                        END IF;
                    END LOOP;

                    INSERT INTO %s(operation, new_data)
                    VALUES ('UPDATE', diff);

                    RETURN NEW;
                END;
                RETURN NEW;
            ELSIF (TG_OP = 'DELETE') THEN
                INSERT INTO %s(operation, old_data)
                VALUES ('DELETE', oldjson);
                RETURN OLD;
            END IF;
            RETURN NULL;
        END;
        $_$ LANGUAGE plpgsql;
    $sql$, audit_trigger_function_name, audit_table_name, audit_table_name, audit_table_name);
    EXECUTE create_trigger_function_sql;

    -- Step 3: Create the trigger
    create_trigger_sql := format($sql$
        DROP TRIGGER IF EXISTS %I ON %s;
        CREATE TRIGGER %I
        AFTER INSERT OR UPDATE OR DELETE ON %s
        FOR EACH ROW
        EXECUTE FUNCTION %s();
    $sql$, audit_trigger_name, target_table, audit_trigger_name, target_table, audit_trigger_function_name);
    EXECUTE create_trigger_sql;

END;
$$ LANGUAGE plpgsql;

CREATE TABLE boxes (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    name TEXT NOT NULL,

    reserved_by TEXT
);
SELECT create_audit_for_table('boxes'::regclass);

CREATE TABLE items (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    box_id int REFERENCES boxes(id) ON DELETE SET NULL,

    count int NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL
);
SELECT create_audit_for_table('items'::regclass);

CREATE TABLE item_reservations (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    item_id int NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    count int NOT NULL,
    reserved_by TEXT NOT NULL
);
SELECT create_audit_for_table('item_reservations'::regclass);

CREATE TABLE labels (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    name TEXT
);
SELECT create_audit_for_table('labels'::regclass);

CREATE TABLE item_labels (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    item_id int NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    label_id int NOT NULL REFERENCES labels(id) ON DELETE CASCADE
);
SELECT create_audit_for_table('item_labels'::regclass);
