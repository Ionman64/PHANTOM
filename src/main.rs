extern crate project_analyser;

use project_analyser::downloader;

fn main() {
    project_analyser::setup_logger().expect("Could not setup logger");
    let projects = downloader::read_project_urls_from_file();

    for project in &projects[5..10] {
        downloader::clone_project(project);
    }

    //downloader::clone_project(&downloader::GitHubProject::new(0, "urlllll".to_string()));
}