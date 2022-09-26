// @generated automatically by Diesel CLI.

diesel::table! {
    boards (id) {
        id -> Int4,
        title -> Text,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    files (id) {
        id -> Int8,
        filename -> Text,
        size -> Int4,
        width -> Int4,
        height -> Int4,
        extension -> Text,
        post_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    posters (id) {
        id -> Int8,
        ip -> Inet,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int8,
        num -> Int8,
        thread_id -> Int8,
        poster_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        body -> Nullable<Text>,
    }
}

diesel::table! {
    threads (id) {
        id -> Int8,
        subject -> Nullable<Text>,
        board_id -> Int4,
        status -> Int2,
        sticky -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(files -> posts (post_id));
diesel::joinable!(posts -> posters (poster_id));
diesel::joinable!(posts -> threads (thread_id));
diesel::joinable!(threads -> boards (board_id));

diesel::allow_tables_to_appear_in_same_query!(
    boards,
    files,
    posters,
    posts,
    threads,
);
