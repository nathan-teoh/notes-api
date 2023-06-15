// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Integer,
        file_path -> Text,
        view_count -> Integer,
        mark_delete -> Integer,
    }
}
