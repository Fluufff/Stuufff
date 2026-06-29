// @generated automatically by Diesel CLI.

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
    labels (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    places (id) {
        id -> Int4,
        in_place -> Nullable<Int4>,
        name -> Text,
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
        description -> Text,
    }
}

diesel::joinable!(reserved_things -> things (thing_id));
diesel::joinable!(thing_labels -> labels (label_id));
diesel::joinable!(thing_labels -> things (thing_id));
diesel::joinable!(things -> places (in_place));

diesel::allow_tables_to_appear_in_same_query!(
    audit_labels,
    audit_places,
    audit_reserved_things,
    audit_thing_labels,
    audit_things,
    labels,
    places,
    reserved_things,
    thing_labels,
    things,
);
