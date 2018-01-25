extern crate project_analyser;

use project_analyser::downloader;
use std::thread;

fn main() {
    project_analyser::setup_logger().expect("Could not setup logger");
    let projects = downloader::read_project_urls_from_file();
    let mut thread_pool = vec![];
    //let (tx, rx)  = mpsc::channel();
    for project in projects.into_iter().skip(299) {
        thread_pool.push(thread::spawn(move || {
            println!("Spawned new thread!");
            downloader::clone_project(project);
        }));
    }
    for thread in thread_pool {
        thread.join();
    }
    //downloader::clone_project(&downloader::GitHubProject::new(0, "urlllll".to_string()));
}
