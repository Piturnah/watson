use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{modules, problems, topics};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = problems)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Problem {
    pub id: i32,
    pub body: String,
    pub author: Option<String>,
    pub source: Option<String>,
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
    #[serde(flatten)]
    pub problem: InsertProblem,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = problems)]
pub struct InsertProblem {
    pub body: String,
    pub author: Option<String>,
    pub source: Option<String>,
    pub solnlink: Option<String>,
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
pub struct Topic {
    pub id: i32,
    pub module_id: i32,
    pub title: String,
}
