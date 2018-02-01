extern crate project_analyser;

use project_analyser::downloader;
use project_analyser::git_analyser;
use project_analyser::models::{GitRepository, ClonedProject};
use project_analyser::downloader::get_home_dir_path;
use std::path::Path;
use project_analyser::utils;

#[test]
fn check_euclidean_distance_1() {
    assert_eq!(utils::two_dimensions_euclidean_distance((1.0,1.0), (2.0,2.0)), 1.4142135623730951);
}

#[test]
fn character_count_test_comma() {
    assert_eq!(downloader::character_count(&String::from(",,,,"), ','), 4);
}

#[test]
fn character_count_test_number_as_param() {
    assert_eq!(downloader::character_count(&String::from(",,,,"), '4'), 0);
}

#[test]
fn character_count_test_russian() {
    assert_eq!(downloader::character_count(&String::from("ру́сский язы́к"), 'й'), 1);
}

#[test]
fn character_count_test_japanese() {
    assert_eq!(downloader::character_count(&String::from("日本語"), '語'), 1);
}

#[test]
fn character_count_test_null_unicode() {
    assert_eq!(downloader::character_count(&String::from(""), '\0'), 0);
}

#[test]
fn clone_project_correct_url() {
    let project = GitRepository { id: 1, url: String::from("https://github.com/bitcoin/bitcoin") };
    assert!(downloader::clone_project(project).is_ok());
}

#[test]
fn clone_project_incorrect_url() {
    let project = GitRepository { id: 1, url: String::from("") };
    assert!(downloader::clone_project(project).is_err());
}

#[test]
fn clone_project_incorrect_url_2() {
    let project = GitRepository { id: 1, url: String::from("https://github.com/private/private") };
    assert!(downloader::clone_project(project).is_err());
}

#[test]
fn check_http_code_correct_url() {
    assert!(downloader::check_url_http_code(200, &String::from("https://github.com/bitcoin/bitcoin")).is_ok());
}

#[test]
fn check_http_code_incorrect_url() {
    assert!(downloader::check_url_http_code(200, &String::from("https://github.com/private/private")).is_err());
}

#[test]
fn check_http_code_incorrect_url_2() {
    assert!(downloader::check_url_http_code(200, &String::from("https://github.com/somewhere/over/the/rainbow")).is_err());
}

#[test]
fn check_http_code_empty_url() {
    assert!(downloader::check_url_http_code(200, &String::from("")).is_err());
}

#[test]
fn check_http_code_null_url() {
    assert!(downloader::check_url_http_code(200, &String::from("\0")).is_err());
}

#[test]
fn check_http_code_garbage_url() {
    assert!(downloader::check_url_http_code(200, &String::from("23456£$%^23456\"")).is_err());
}

#[test]
fn utf8_to_http_code_correct_code_404() {
    let mut input = vec![0x34, 0x30, 0x34];
    let result = match downloader::utf8_to_http_code(input) {
        Ok(result) => result,
        Err(_) => -1,
    };
    assert_eq!(result, 404);
}

#[test]
fn utf8_to_http_code_correct_code_200() {
    let mut input = vec![0x032, 0x030, 0x030];
    let result = match downloader::utf8_to_http_code(input) {
        Ok(result) => result,
        Err(_) => -1,
    };
    assert_eq!(result, 200);
}

#[test]
fn utf8_to_http_code_text() {
    let mut input = vec![0x60, 0x60, 0x60, 0x65, 0x65, 0x65, 0x23, 0x43];
    let result = match downloader::utf8_to_http_code(input) {
        Ok(result) => false,
        Err(_) => true,
    };
    assert!(result);
}

#[test]
fn utf8_to_http_code_code_large() {
    let mut input = vec![0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32];
    let result = match downloader::utf8_to_http_code(input) {
        Ok(result) => false,
        Err(_) => true,
    };
    assert!(result);
}

//The following code tests functionality that will be changed soon.
#[test]
fn read_project_urls_should_pass() {
    let result = match downloader::read_project_urls_from_file(String::from("projects.csv")) {
        Ok(_) => { true },
        Err(_) => { false },
    };
    assert!(result);
}

#[test]
#[should_panic]
fn read_project_urls_should_panic() {
    downloader::read_project_urls_from_file(String::from("23453$^%£$dFGSf.csv"));
}

#[should_panic]
fn read_project_urls_should_panic_2() {
    downloader::read_project_urls_from_file(String::from("23453$^%£$dFGSf.csv"));
}
//TODO: remove above tests

#[test]
fn read_commits_per_day_correct_url() {
    let github_project = GitRepository {id:7, url:String::from("https://github.com/bitcoin/bitcoin")};
    let home_path = get_home_dir_path().expect("Could not get home directory");
    let project_path = Path::new(&home_path)
        .join(String::from("project_analyser"))
        .join(String::from("repos"))
        .join(github_project.id.to_string());
    let cloned_project = ClonedProject::new(github_project, project_path);
    let result = match git_analyser::count_commits_per_day(&cloned_project) {
        Ok(date_count) => true,
        Err(_) => false,
    };
    assert!(result);
}