mod models;
mod schema;

use std::{env, net::SocketAddr};

use axum::{
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use models::{NewProblem, Problem};

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

    // TODO: dotenv
    let origins = [
        "http://localhost:5173".parse().unwrap(),
        "http://localhost:4173".parse().unwrap(),
    ];

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/problems/create", post(create_problem))
        .layer(CorsLayer::new().allow_methods(Any).allow_headers(Any).allow_origin(origins));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_problem(
    Json(new_problem): Json<NewProblem>,
) -> Result<Json<Problem>, (StatusCode, String)> {
    use crate::schema::problems;
    let mut conn = establish_connection();
    diesel::insert_into(problems::table)
        .values(&new_problem)
        .returning(Problem::as_returning())
        .get_result(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        .map(|res| Json(res))
}
