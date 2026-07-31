-- Your SQL goes here

CREATE TABLE departments (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    name TEXT NOT NULL,
    main_img UUID
);
SELECT create_audit_for_table('departments'::regclass);

CREATE TABLE department_images (
    -- id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel
    department_id int NOT NULL REFERENCES departments(id) ON DELETE CASCADE,
    id UUID NOT NULL PRIMARY KEY
);
ALTER TABLE departments ADD FOREIGN KEY (main_img) REFERENCES department_images(id) ON DELETE SET NULL;
SELECT create_audit_for_table('department_images'::regclass);

CREATE TABLE places (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    in_place int REFERENCES places(id) ON DELETE SET NULL,
    in_department int REFERENCES departments(id) ON DELETE SET NULL,

    name TEXT NOT NULL,
    description TEXT,
    main_img UUID
);
SELECT create_audit_for_table('places'::regclass);

CREATE TABLE things (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    count int NOT NULL,
    in_place int REFERENCES places(id) ON DELETE SET NULL,

    name TEXT NOT NULL,
    description TEXT,
    main_img UUID
);
SELECT create_audit_for_table('things'::regclass);

CREATE TABLE labels (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    color TEXT,
    name TEXT NOT NULL,
    description TEXT
);
SELECT create_audit_for_table('labels'::regclass);

CREATE TABLE thing_labels (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    thing_id int NOT NULL REFERENCES things(id) ON DELETE CASCADE,
    label_id int NOT NULL REFERENCES labels(id) ON DELETE CASCADE
);
SELECT create_audit_for_table('thing_labels'::regclass);

CREATE TABLE thing_images (
    -- id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel
    thing_id int NOT NULL REFERENCES things(id) ON DELETE CASCADE,
    id UUID NOT NULL PRIMARY KEY
);
ALTER TABLE things ADD FOREIGN KEY (main_img) REFERENCES thing_images(id) ON DELETE SET NULL;
SELECT create_audit_for_table('thing_images'::regclass);

CREATE TABLE place_images (
    -- id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel
    place_id int NOT NULL REFERENCES places(id) ON DELETE CASCADE,
    id UUID NOT NULL PRIMARY KEY
);
ALTER TABLE places ADD FOREIGN KEY (main_img) REFERENCES place_images(id) ON DELETE SET NULL;
SELECT create_audit_for_table('place_images'::regclass);

CREATE TABLE reserved_things (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    thing_id int NOT NULL REFERENCES things(id) ON DELETE CASCADE,
    count int NOT NULL,

    reserved_by TEXT NOT NULL
);
SELECT create_audit_for_table('reserved_things'::regclass);

CREATE TABLE reserved_places (
    id int GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- unused, required by diesel

    place_id int NOT NULL REFERENCES places(id) ON DELETE CASCADE,

    reserved_by TEXT NOT NULL
);
SELECT create_audit_for_table('reserved_places'::regclass);

INSERT INTO departments (name) VALUES
('IT'),
('Bar'),
('Charity'),
('Dealers Den'),
('Entertainment'),
('Feedback'),
('Fursuit Affairs'),
('Human Relations'),
('Logistics'),
('Media'),
('Public Relations'),
('Registration & Venue'),
('Stage'),
('Stewards and Safety'),
('Theming');

INSERT INTO places (name) VALUES
('Spaaaaace');
INSERT INTO places (name, in_place) VALUES
('Earth', 1),
('The Container', 2),
('At Jura & Woof''s', 2);
INSERT INTO places (name, in_place, in_department) VALUES
('IT Box 1', 4, 1),
('Bar Box 1', 3, 2);

INSERT INTO labels (color, name, description) VALUES
('red', 'needs recount', 'needs to be recounted');

-- INSERT INTO things (count, in_place, name, description) VALUES
-- (1, 4, 'USB Keyboard US + USB mouse', 'a'),
-- (6, 5, 'Mojito glass', 'Like IKEA''s glasses but stronger');

-- INSERT INTO thing_labels (thing_id, label_id) VALUES
-- (2, 1);
