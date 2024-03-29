use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{
    access_tokens, modules, problem_topic, problems, solutions, topics, user_problem, users,
};

#[derive(Identifiable, Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = access_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccessToken {
    pub id: Uuid,
    pub name: String,
    pub redeemed: bool,
}

#[derive(Identifiable, Queryable, Selectable, Associations, Serialize, Debug, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = problems)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Problem {
    pub id: i32,
    pub body: Option<String>,
    pub author: Option<String>,
    pub source: Option<String>,
    pub solnlink: Option<String>,
    pub submitted_at: NaiveDateTime,
    pub user_id: Option<Uuid>,
    pub img_path: Option<String>,
}

#[derive(
    Identifiable, Queryable, Selectable, Serialize, Deserialize, Associations, Debug, Clone,
)]
#[diesel(belongs_to(Problem))]
#[diesel(belongs_to(User))]
#[diesel(table_name = solutions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Solution {
    pub id: i32,
    pub problem_id: i32,
    pub body: Option<String>,
    pub submitted_at: NaiveDateTime,
    pub user_id: Option<Uuid>,
    pub img_path: Option<String>,
}

#[derive(Identifiable, Queryable, Selectable, Associations, Debug)]
#[diesel(belongs_to(Problem))]
#[diesel(belongs_to(Topic))]
#[diesel(primary_key(problem_id, topic_id))]
#[diesel(table_name = problem_topic)]
#[diesel(primary_key(problem_id, topic_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProblemTopic {
    pub problem_id: i32,
    pub topic_id: i32,
}
#[derive(Identifiable, Queryable, Selectable, Insertable, Associations, Debug)]
#[diesel(belongs_to(Problem))]
#[diesel(belongs_to(User))]
#[diesel(primary_key(user_id, problem_id))]
#[diesel(table_name = user_problem)]
#[diesel(primary_key(user_id, problem_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProblem {
    pub user_id: Uuid,
    pub problem_id: i32,
    pub last_solved: NaiveDateTime,
    pub successful: bool,
}

#[derive(Identifiable, Queryable, Selectable, Debug)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub name: String,
    pub email: String,
    pub id: Uuid,
    pub password: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AddModule {
    New(String),
    Existing(i32),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AddTopic {
    New(String),
    Existing(i32),
}

#[derive(Deserialize)]
pub struct NewProblem {
    pub module: AddModule,
    pub topic: AddTopic,
    pub soln: Option<String>,
    pub soln_img: Option<String>,
    #[serde(flatten)]
    pub problem: InsertProblem,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = problems)]
pub struct InsertProblem {
    pub body: Option<String>,
    pub author: Option<String>,
    pub source: Option<String>,
    pub solnlink: Option<String>,
    pub img_path: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = modules)]
pub struct InsertModule {
    pub title: String,
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Hash, Eq, PartialEq, Debug)]
#[diesel(table_name = modules)]
pub struct Module {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize)]
pub struct ModulesView {
    pub modules: Vec<Module>,
    pub topics: Vec<Topic>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Debug)]
#[diesel(belongs_to(Module))]
#[diesel(table_name = topics)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Topic {
    pub id: i32,
    pub module_id: i32,
    pub title: String,
}
