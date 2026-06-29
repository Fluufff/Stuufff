-- Your SQL goes here

CREATE TABLE places (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    in_place int REFERENCES places(id) ON DELETE SET NULL,

    name TEXT NOT NULL
);
SELECT create_audit_for_table('places'::regclass);

CREATE TABLE things (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    count int NOT NULL,
    in_place int REFERENCES places(id) ON DELETE SET NULL,

    name TEXT NOT NULL,
    description TEXT NOT NULL
);
SELECT create_audit_for_table('things'::regclass);

CREATE TABLE labels (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    name TEXT NOT NULL
);
SELECT create_audit_for_table('labels'::regclass);

CREATE TABLE thing_labels (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    thing_id int NOT NULL REFERENCES things(id) ON DELETE CASCADE,
    label_id int NOT NULL REFERENCES labels(id) ON DELETE CASCADE
);
SELECT create_audit_for_table('thing_labels'::regclass);

CREATE TABLE reserved_things (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    thing_id int NOT NULL REFERENCES things(id) ON DELETE CASCADE,
    count int NOT NULL,

    reserved_by TEXT NOT NULL
);
SELECT create_audit_for_table('reserved_things'::regclass);
