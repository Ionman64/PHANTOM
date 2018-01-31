extern crate diesel;
extern crate dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use self::dotenv::dotenv;
use std::env;
use super::models::{NewGitHubProject, GitHubProject};
use std::io::ErrorKind;



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
 /*
pub fn show_posts() {
    use super::schema::github_projects::dsl::*;

    let connection = establish_connection();
    let results = github_projects
        .limit(300)
        .load::<GitHubProject>(&connection)
        .expect("Error loading projects");

    println!("Displaying {} projects", results.len());
    for post in results {
        println!("{}", post.id);
        println!("----------");
        println!("{}", post.url);
    }
}
*/

pub fn insert_new_project(project: &NewGitHubProject) -> Result<GitHubProject, ErrorKind> {
    use schema::github_projects;
    let connection = establish_connection();
    let inserted_project:GitHubProject = match diesel::insert_into(github_projects::table)
        .values(project)
        .get_result(&connection) {
        Ok(x) => x,
        Err(_) => return Err(ErrorKind::AlreadyExists),
    };
    Ok(inserted_project)
}

pub fn get_project_by_url(url_str: String) -> Result<GitHubProject, ErrorKind> {
    use super::schema::github_projects::dsl::*;

    let connection = establish_connection();
    match github_projects
        .filter(url.eq(url_str))
        .first(&connection) {
        Ok(x) => Ok(x),
        Err(_) => Err(ErrorKind::NotFound)
    }
}

mod commit_frequency {

}

mod git_repository {

    pub fn create() {

    }

    pub fn read() {

    }

    pub fn update() {

    }

    pub fn delete() {

    }
}
