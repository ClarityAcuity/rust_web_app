use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

// TODO a better mechanism for setting the path to the database
pub fn establish_connection() -> SqliteConnection {
    let db = "./backend/testdb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

// TODO a better job of error handling
pub fn create_task<'a>(connection: &SqliteConnection, title: &'a str) {
    let task = models::NewTask { title };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading tasks")
}
