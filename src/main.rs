extern crate project_analyser;
extern crate fern;
#[macro_use]
extern crate log;

use project_analyser::downloader;
use project_analyser::thread_helper::ThreadPool;
use project_analyser::downloader::{get_home_dir_path};
use project_analyser::git_analyser;
use std::path::Path;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    use project_analyser::downloader::*;
    use project_analyser::git_analyser::*;
    use std::path::PathBuf;
    use std::str;
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
        let project = GitHubProject {id:1, url:String::from("https://github.com/bitcoin/bitcoin")};
        assert!(downloader::clone_project(project).is_ok());
    }

    #[test]
    fn clone_project_incorrect_url() {
        let project = GitHubProject {id:1, url:String::from("")};
        assert!(downloader::clone_project(project).is_err());
    }

    #[test]
    fn clone_project_incorrect_url_2() {
        let project = GitHubProject {id:1, url:String::from("https://github.com/private/private")};
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
            Ok(_) => {true},
            Err(_) => {false},
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
    fn read_commits_per_day() {
        let github_project = GitHubProject::new("https://github.com/bitcoin/bitcoin", 7);
        let home_path = get_home_dir_path().expect("Could not get home directory");
        let project_path = Path::new(&home_path)
        .join(String::from("project_analyser"))
        .join(String::from("repos"))
        .join(project.id.to_string());
        let cloned_project = ClonedProject::new(github_project, project_path);
        let result = match git_analyser::count_commits_per_day(&cloned_project) {
            Ok(date_count) => { date_count },
            Err(_) => {
                error!("Could not count commits");
                return
            },
        };
        assert_eq!(result, );
    }






}

fn main() {
    match project_analyser::setup_logger() {
        Ok(_) => {},
        Err(_) => { panic!("Cannot setup logger, Programme will terminate") }
    };

    let projects = match downloader::read_project_urls_from_file(String::from("projects.csv")) {
        Ok(project_struct) => {
            info!("Finished reading projects file;");
            match project_struct.skipped_lines {
                None => {
                    info!("No lines skipped");
                },
                Some(lines) => {
                    warn!("Lines Skipped:");
                    for line in lines {
                        warn!("{}", line);
                    }
                },
            }
            project_struct.response
        }
        Err(_) => {
            panic!("Could not read the project URLs");
        }
    };
    let csv_path = Path::new(&get_home_dir_path().unwrap())
        .join("project_analyser")
        .join("analysis");
    fs::create_dir_all(&csv_path).expect("Could not create directories");

    let thread_pool = ThreadPool::new(75);
    for project in projects.into_iter() {
        thread_pool.execute(move || {
            println!("Spawned new thread!");
            let cloned_project = match downloader::clone_project(project) {
                Ok(cloned_project) => cloned_project,
                Err(_) => {
                    error!("Could not clone project");
                    return;
                },
            };
            match git_analyser::generate_git_log(&cloned_project) {
                Ok(log) => {
                    info!("Created log in {}", &cloned_project.path);
                    log
                }
                Err(e) => {
                    error!("Could not generate log file for project {}: {:?}", &cloned_project.path, e);
                    return
                }
            };
            let date_count = match git_analyser::count_commits_per_day(&cloned_project) {
                Ok(date_count) => { date_count },
                Err(_) => {
                    error!("Could not count commits");
                    return
                },
            };
            match git_analyser::generate_analysis_csv(&cloned_project, date_count) {
                Ok(_) => {},
                Err(_) => { error!("Could not generate analysis CSV") },
            }
        });
    }
}
