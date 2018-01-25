extern crate project_analyser;

use project_analyser::downloader;
use project_analyser::thread_helper::ThreadPool;

fn main() {
    project_analyser::setup_logger().expect("Could not setup logger");

    let projects = downloader::read_project_urls_from_file();

    let thread_pool = ThreadPool::new(50);
    for project in projects.into_iter() {
        thread_pool.execute(move || {
            downloader::clone_project(project);
        });

    }
}

