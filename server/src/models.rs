use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{modules, problems};

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
