// @generated automatically by Diesel CLI.

diesel::table! {
    problems (id) {
        id -> Integer,
        body -> Text,
        soln -> Text,
        source -> Nullable<Text>,
        author -> Nullable<Text>,
        soln_author -> Nullable<Text>,
    }
}
