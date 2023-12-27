mod auth;
mod models;
mod schema;

use std::{env, error::Error, net::SocketAddr};

use axum::{
    http::StatusCode,
    middleware,
    response::Json,
    routing::{get, post},
    Router,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};

use models::{Module, ModulesView, NewProblem, Problem, Topic};

use crate::models::{AddModule, AddTopic, InsertModule};

// The migration path is relative to `CARGO_MANIFEST_DIR`.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() {
    let mut conn = establish_connection();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    // TODO: dotenv.
    let origins = [
        "http://localhost:5173".parse().unwrap(),
        "http://localhost:4173".parse().unwrap(),
    ];

    let app = Router::new()
        .route("/problems/create", post(create_problem))
        .route("/problems/request", get(request_problem))
        .route("/modules", get(get_modules))
        .route_layer(middleware::from_fn(auth::auth))
        .route("/login", post(auth::login))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_origin(origins),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_modules() -> Result<Json<ModulesView>, (StatusCode, String)> {
    use schema::{modules, topics};
    let mut conn = establish_connection();
    let modules = modules::table
        .select(Module::as_select())
        .get_results(&mut conn)
        .map_err(internal_error)?;
    let topics = topics::table
        .select(Topic::as_select())
        .get_results(&mut conn)
        .map_err(internal_error)?;

    Ok(Json(ModulesView { modules, topics }))
}

async fn create_problem(
    Json(new_problem): Json<NewProblem>,
) -> Result<Json<Problem>, (StatusCode, String)> {
    use schema::{modules, problem_topic, problems, solutions, topics};
    let mut conn = establish_connection();
    let module_id = match new_problem.module {
        AddModule::Existing(id) => id,
        AddModule::New(title) => diesel::insert_into(modules::table)
            .values(InsertModule { title })
            .returning(modules::id)
            .get_result(&mut conn)
            .map_err(internal_error)?,
    };
    let topic_id = match new_problem.topic {
        AddTopic::Existing(id) => id,
        AddTopic::New(title) => diesel::insert_into(topics::table)
            .values((topics::module_id.eq(module_id), topics::title.eq(title)))
            .returning(topics::id)
            .get_result(&mut conn)
            .map_err(internal_error)?,
    };

    let result = diesel::insert_into(problems::table)
        .values(&new_problem.problem)
        .returning(Problem::as_returning())
        .get_result(&mut conn)
        .map_err(internal_error)?;

    if let Some(soln) = new_problem.soln {
        diesel::insert_into(solutions::table)
            .values((
                solutions::body.eq(soln),
                solutions::problem_id.eq(result.id),
            ))
            .execute(&mut conn)
            .map_err(internal_error)?;
    }

    diesel::insert_into(problem_topic::table)
        .values((
            problem_topic::problem_id.eq(result.id),
            problem_topic::topic_id.eq(topic_id),
        ))
        .execute(&mut conn)
        .map_err(internal_error)?;

    Ok(Json(result))
}

#[derive(Deserialize, Debug)]
struct ProblemRequest {
    topic_ids: Vec<i32>,
}

async fn request_problem(
    Json(request): Json<ProblemRequest>,
) -> Result<Json<Problem>, (StatusCode, String)> {
    todo!()
}

pub fn internal_error<E: Error>(error: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
