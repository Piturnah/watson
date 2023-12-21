// @generated automatically by Diesel CLI.

diesel::table! {
    problems (id) {
        id -> Int4,
        body -> Text,
        author -> Nullable<Varchar>,
        source -> Nullable<Varchar>,
    }
}
