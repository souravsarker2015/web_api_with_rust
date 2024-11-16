// @generated automatically by Diesel CLI.

diesel::table! {
    rustacean (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    test (id) {
        id -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    rustacean,
    test,
);
