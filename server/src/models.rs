use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::problems;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = problems)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Problem {
    pub id: i32,
    pub body: String,
    pub author: Option<String>,
    pub source: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = problems)]
pub struct NewProblem {
    pub body: String,
}
