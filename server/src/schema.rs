// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (id) {
        id -> Uuid,
        name -> Varchar,
        redeemed -> Bool,
    }
}

diesel::table! {
    modules (id) {
        id -> Int4,
        title -> Varchar,
    }
}

diesel::table! {
    problem_topic (problem_id, topic_id) {
        problem_id -> Int4,
        topic_id -> Int4,
    }
}

diesel::table! {
    problems (id) {
        id -> Int4,
        body -> Nullable<Text>,
        author -> Nullable<Varchar>,
        source -> Nullable<Varchar>,
        solnlink -> Nullable<Varchar>,
        submitted_at -> Timestamp,
        user_id -> Nullable<Uuid>,
        img_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    solutions (id) {
        id -> Int4,
        problem_id -> Int4,
        body -> Nullable<Text>,
        author -> Nullable<Varchar>,
        submitted_at -> Timestamp,
        user_id -> Nullable<Uuid>,
        img_path -> Nullable<Varchar>,
    }
}

diesel::table! {
    topics (id) {
        id -> Int4,
        module_id -> Int4,
        title -> Varchar,
    }
}

diesel::table! {
    user_problem (user_id, problem_id) {
        user_id -> Uuid,
        problem_id -> Int4,
        last_solved -> Timestamp,
        successful -> Bool,
    }
}

diesel::table! {
    users (id) {
        name -> Varchar,
        email -> Varchar,
        id -> Uuid,
        password -> Nullable<Varchar>,
    }
}

diesel::joinable!(problem_topic -> problems (problem_id));
diesel::joinable!(problem_topic -> topics (topic_id));
diesel::joinable!(problems -> users (user_id));
diesel::joinable!(solutions -> problems (problem_id));
diesel::joinable!(solutions -> users (user_id));
diesel::joinable!(topics -> modules (module_id));
diesel::joinable!(user_problem -> problems (problem_id));
diesel::joinable!(user_problem -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    access_tokens,
    modules,
    problem_topic,
    problems,
    solutions,
    topics,
    user_problem,
    users,
);
