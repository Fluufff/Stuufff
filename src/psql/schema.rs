// @generated automatically by Diesel CLI.

diesel::table! {
    audit_boxes (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_item_labels (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_item_reservations (audit_id) {
        audit_id -> Int8,
        operation -> Text,
        changed_at -> Nullable<Timestamptz>,
        changed_by -> Nullable<Text>,
        old_data -> Nullable<Jsonb>,
        new_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    audit_items (audit_id) {
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
    boxes (id) {
        id -> Int4,
        name -> Text,
        reserved_by -> Nullable<Text>,
    }
}

diesel::table! {
    item_labels (id) {
        id -> Int4,
        item_id -> Int4,
        label_id -> Int4,
    }
}

diesel::table! {
    item_reservations (id) {
        id -> Int4,
        item_id -> Int4,
        count -> Int4,
        reserved_by -> Text,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        box_id -> Nullable<Int4>,
        count -> Int4,
        title -> Text,
        description -> Text,
    }
}

diesel::table! {
    labels (id) {
        id -> Int4,
        name -> Nullable<Text>,
    }
}

diesel::joinable!(item_labels -> items (item_id));
diesel::joinable!(item_labels -> labels (label_id));
diesel::joinable!(item_reservations -> items (item_id));
diesel::joinable!(items -> boxes (box_id));

diesel::allow_tables_to_appear_in_same_query!(
    audit_boxes,
    audit_item_labels,
    audit_item_reservations,
    audit_items,
    audit_labels,
    boxes,
    item_labels,
    item_reservations,
    items,
    labels,
);
