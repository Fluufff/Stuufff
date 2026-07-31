// @generated automatically by Diesel CLI.

diesel::table! {
    audit_department_images (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_departments (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_labels (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_place_images (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_places (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_reserved_places (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_reserved_things (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_thing_images (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_thing_labels (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_things (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    department_images (id) {
        department_id -> Int4,
        id -> Uuid,
    }
}

diesel::table! {
    departments (id) {
        id -> Int4,
        name -> Text,
        main_img -> Nullable<Uuid>,
    }
}

diesel::table! {
    labels (id) {
        id -> Int4,
        color -> Nullable<Text>,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    place_images (id) {
        place_id -> Int4,
        id -> Uuid,
    }
}

diesel::table! {
    places (id) {
        id -> Int4,
        in_place -> Nullable<Int4>,
        in_department -> Nullable<Int4>,
        name -> Text,
        description -> Nullable<Text>,
        main_img -> Nullable<Uuid>,
    }
}

diesel::table! {
    reserved_places (id) {
        id -> Int4,
        place_id -> Int4,
        reserved_by -> Text,
    }
}

diesel::table! {
    reserved_things (id) {
        id -> Int4,
        thing_id -> Int4,
        count -> Int4,
        reserved_by -> Text,
    }
}

diesel::table! {
    thing_images (id) {
        thing_id -> Int4,
        id -> Uuid,
    }
}

diesel::table! {
    thing_labels (id) {
        id -> Int4,
        thing_id -> Int4,
        label_id -> Int4,
    }
}

diesel::table! {
    things (id) {
        id -> Int4,
        count -> Int4,
        in_place -> Nullable<Int4>,
        name -> Text,
        description -> Nullable<Text>,
        main_img -> Nullable<Uuid>,
    }
}

diesel::joinable!(places -> departments (in_department));
diesel::joinable!(reserved_places -> places (place_id));
diesel::joinable!(reserved_things -> things (thing_id));
diesel::joinable!(thing_labels -> labels (label_id));
diesel::joinable!(thing_labels -> things (thing_id));
diesel::joinable!(things -> places (in_place));

diesel::allow_tables_to_appear_in_same_query!(
    audit_department_images,
    audit_departments,
    audit_labels,
    audit_place_images,
    audit_places,
    audit_reserved_places,
    audit_reserved_things,
    audit_thing_images,
    audit_thing_labels,
    audit_things,
    department_images,
    departments,
    labels,
    place_images,
    places,
    reserved_places,
    reserved_things,
    thing_images,
    thing_labels,
    things,
);
