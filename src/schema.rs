// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Integer,
        url_id -> Text,
        long_url -> Text,
    }
}
