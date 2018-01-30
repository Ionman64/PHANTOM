extern crate diesel;
extern crate dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use self::dotenv::dotenv;
use std::env;
use super::models::GitHubProject;



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn show_posts() {
    use super::schema::github_projects::dsl::*;

    let connection = establish_connection();
    let results = github_projects
        .limit(10)
        .load::<GitHubProject>(&connection)
        .expect("Error loading projects");

    println!("Displaying {} projects", results.len());
    for post in results {
        println!("{}", post.id);
        println!("----------");
        println!("{}", post.url);
    }
}
