mod auth;
mod models;
mod schema;

use std::{cmp::Ordering, collections::HashMap, env, error::Error, net::SocketAddr, sync::Arc};

use axum::{
    http::{HeaderMap, StatusCode},
    middleware,
    response::Json,
    routing::{get, post, put},
    Router,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{pg::PgConnection, prelude::*, query_dsl::BelongingToDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use rand::{distributions::WeightedIndex, prelude::*};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use models::{Module, ModulesView, NewProblem, Problem, Topic};

use crate::models::{AddModule, AddTopic, InsertModule, ProblemTopic, Solution, UserProblem};

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

    let sessions = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/problems/create", post(create_problem))
        .route("/problems/request", post(request_problem))
        .route("/problems/solve", put(solve_problem))
        .route("/solutions", post(submit_solution))
        .route("/modules", get(get_modules))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&sessions),
            auth::auth,
        ))
        .route("/login", post(auth::login))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_origin(origins),
        )
        .with_state(sessions);
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

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = schema::solutions)]
struct SubmitSolution {
    problem_id: i32,
    body: String,
}

async fn submit_solution(
    headers: HeaderMap,
    Json(solution): Json<SubmitSolution>,
) -> Result<(), (StatusCode, String)> {
    use schema::solutions::*;
    let mut conn = establish_connection();
    let user = headers
        .get("user_sub")
        .ok_or((StatusCode::UNAUTHORIZED, "No user".to_string()))?
        .to_str()
        .map_err(internal_error)?;
    diesel::insert_into(table)
        .values((solution, submitted_by.eq(user.to_string())))
        .execute(&mut conn)
        .map_err(internal_error)?;
    Ok(())
}

async fn create_problem(
    headers: HeaderMap,
    Json(new_problem): Json<NewProblem>,
) -> Result<Json<Problem>, (StatusCode, String)> {
    use schema::{modules, problem_topic, problems, solutions, topics};
    let mut conn = establish_connection();
    let user = headers
        .get("user_sub")
        .ok_or((StatusCode::UNAUTHORIZED, "No user".to_string()))?
        .to_str()
        .map_err(internal_error)?;

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
        .values((
            &new_problem.problem,
            problems::submitted_by.eq(user.to_string()),
        ))
        .returning(Problem::as_returning())
        .get_result(&mut conn)
        .map_err(internal_error)?;

    if let Some(soln) = new_problem.soln {
        diesel::insert_into(solutions::table)
            .values((
                solutions::body.eq(soln),
                solutions::problem_id.eq(result.id),
                solutions::submitted_by.eq(user.to_string()),
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

#[derive(Deserialize)]
struct SolveProblem {
    problem_id: i32,
    successful: bool,
}

async fn solve_problem(
    headers: HeaderMap,
    Json(SolveProblem {
        problem_id,
        successful,
    }): Json<SolveProblem>,
) -> Result<(), (StatusCode, String)> {
    use schema::user_problem;
    let mut conn = establish_connection();
    let user_id = headers
        .get("user_sub")
        .ok_or((StatusCode::UNAUTHORIZED, "No user".to_string()))?
        .to_str()
        .map_err(internal_error)?
        .to_string();
    diesel::insert_into(user_problem::table)
        .values(UserProblem {
            user_id,
            problem_id,
            last_solved: Utc::now().naive_utc(),
            successful,
        })
        .on_conflict((user_problem::user_id, user_problem::problem_id))
        .do_update()
        .set((
            user_problem::last_solved.eq(Utc::now().naive_utc()),
            user_problem::successful.eq(successful),
        ))
        .execute(&mut conn)
        .map_err(internal_error)?;

    Ok(())
}

#[derive(Deserialize, Debug)]
struct ProblemRequest {
    topic_ids: Vec<i32>,
}

#[derive(Serialize, Debug)]
struct ProblemResponse {
    problem: Option<Problem>,
    solution: Option<String>,
}

async fn request_problem(
    headers: HeaderMap,
    Json(request): Json<ProblemRequest>,
) -> Result<Json<ProblemResponse>, (StatusCode, String)> {
    use schema::{problem_topic, problems, solutions, topics, user_problem, users};
    let mut conn = establish_connection();
    let user = &headers
        .get("user_sub")
        .ok_or((StatusCode::UNAUTHORIZED, "No user".to_string()))?
        .to_str()
        .map_err(internal_error)?;

    let selected_topics: Vec<Topic> = match request.topic_ids.len() {
        0 => topics::table.load(&mut conn),
        _ => topics::table
            .filter(topics::id.eq_any(&request.topic_ids))
            .load(&mut conn),
    }
    .map_err(internal_error)?;
    let mut valid_problems: Vec<(i32, (Option<NaiveDateTime>, Option<bool>), Problem)> =
        ProblemTopic::belonging_to(&selected_topics)
            .inner_join(problems::table.left_join(user_problem::table.inner_join(users::table)))
            .filter(users::id.eq(user).or(users::id.is_null()))
            //.distinct_on(problems::id)
            .select((
                problem_topic::topic_id,
                (
                    // TODO: I don't remember why these have to be optional values.
                    user_problem::last_solved.nullable(),
                    user_problem::successful.nullable(),
                ),
                Problem::as_select(),
            ))
            .load(&mut conn)
            .map_err(internal_error)?;

    valid_problems.sort_by(|(_, (user1, _), problem1), (_, (user2, _), problem2)| {
        match problem1.id.cmp(&problem2.id) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match (user1.is_some(), user2.is_some()) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => Ordering::Equal, // This *should* be unreachable!
            },
        }
    });
    valid_problems.dedup_by_key(|(_, _, problem)| problem.id);

    let mut topic_problems_map: HashMap<i32, Vec<Problem>> = HashMap::new();
    for (topic_id, (last_solved, successful), problem) in valid_problems {
        if let Some(last_solved) = last_solved {
            // Reject this problem if we already saw it too recently.
            if Utc::now()
                .naive_utc()
                .signed_duration_since(last_solved)
                .num_weeks()
                < if successful.unwrap() { 4 } else { 1 }
            {
                continue;
            }
        }

        if let Some(ps) = topic_problems_map.get_mut(&topic_id) {
            ps.push(problem);
        } else {
            topic_problems_map.insert(topic_id, vec![problem]);
        }
    }

    // Now we hopefully have only one of every problem!
    // Next, we have to find the user's success rate for each topic.

    let n_incorrect: Vec<(i32, i64)> = ProblemTopic::belonging_to(&selected_topics)
        .inner_join(problems::table.left_join(user_problem::table.inner_join(users::table)))
        .filter(users::id.eq(user))
        .filter(diesel::dsl::not(user_problem::successful))
        .group_by(problem_topic::topic_id)
        .select((problem_topic::topic_id, diesel::dsl::count(problems::id)))
        .load(&mut conn)
        .map_err(internal_error)?;
    let n_total: Vec<(i32, i64)> = ProblemTopic::belonging_to(&selected_topics)
        .inner_join(problems::table.left_join(user_problem::table.inner_join(users::table)))
        .filter(users::id.eq(user))
        .group_by(problem_topic::topic_id)
        .select((problem_topic::topic_id, diesel::dsl::count(problems::id)))
        .load(&mut conn)
        .map_err(internal_error)?;

    // Laplace's rule of succession.
    let mut laplace_weights = Vec::new();
    for id_k in &request.topic_ids {
        let numerator = n_incorrect
            .iter()
            .find(|(id, _)| *id == *id_k)
            .map(|(_, n)| *n)
            .unwrap_or(0) as f64
            + 1.0;
        let denominator = n_total
            .iter()
            .find(|(id, _)| *id == *id_k)
            .map(|(_, n)| *n)
            .unwrap_or(0) as f64
            + 2.0;
        laplace_weights.push(numerator / denominator);
    }

    let dist = WeightedIndex::new(&laplace_weights).map_err(internal_error)?;
    let mut rng = thread_rng();
    let next_problem = loop {
        let next_topic = request.topic_ids[dist.sample(&mut rng)];
        if let Some(problem) = topic_problems_map.get_mut(&next_topic).and_then(|topic| {
            (topic.len() != 0).then(|| {
                let next_problem_idx: usize = rng.gen_range(0..topic.len());
                topic.swap_remove(next_problem_idx)
            })
        }) {
            break Some(problem.to_owned());
        }
        if topic_problems_map
            .values()
            .map(|ps| ps.len())
            .sum::<usize>()
            == 0
        {
            break None;
        }
    };

    let solution: Option<String> = next_problem.as_ref().and_then(|problem| {
        Solution::belonging_to(problem)
            .select(solutions::body)
            .first(&mut conn)
            .ok()
    });

    Ok(Json(ProblemResponse {
        problem: next_problem,
        solution,
    }))
}

pub fn internal_error<E: Error>(error: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}
