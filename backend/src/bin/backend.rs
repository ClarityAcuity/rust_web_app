#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use backend::db::{establish_connection, query_task};
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use rust_web_app::JsonApiResponse;

/**
 * production app should:
 * testing, of both the unit and integration varieties
 * documentation, from comments to docstrings to REST API user (developer) docs
 * stricter conformance to JSON API
 * API versioning
 * database connection pooling so that we don't have to do establish_connection for every request, which would be important under load
 * make our response object use a parameterized type so that we can return different object types as the API grows new features
 *
 * Exercises:
 * Bring the API in conformance with the JSON API Spec
 * Use Connection Pooling
 */

#[get("/tasks")]
fn tasks_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    let conn = establish_connection();

    for db_task in query_task(&conn) {
        let api_task = rust_web_app::Task {
            id: db_task.id,
            title: db_task.title,
        };
        response.data.push(api_task);
    }

    Json(response)
}

fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![tasks_get])
        .attach(cors)
        .launch();

    Ok(())
}
