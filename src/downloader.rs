use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::Command;
use std::str;
use std::io::ErrorKind;
use models::{ClonedProject, GitRepository};

pub struct LinesResponse<T> {
    pub response: Vec<T>,
    pub skipped_lines: Option<Vec<u32>>,
}


/// Reads  the csv file "projects.csv" (see project root directory) and extracts the id and url for each row.
pub fn read_project_urls_from_file(filepath: String) -> Result<LinesResponse<GitRepository>, ErrorKind> {
    let csv_file = match File::open(filepath) {
        Ok(file) => file,
        Err(_) => { panic!("Could not open urls file") }
    };
    let reader = BufReader::new(csv_file);
    let mut projects: Vec<GitRepository> = Vec::new();
    let skip_rows = 1;
    let mut skipped_lines: Vec<u32> = Vec::new();
    let mut line_num: u32 = 1;
    let mut count = 1;
    for line in reader.lines().skip(skip_rows) {
        line_num += 1;
        let str_line = match line {
            Ok(line) => line,
            Err(_) => {
                warn!("Could not read line {}", line_num);
                skipped_lines.push(line_num);
                continue;
            }
        };

        if character_count(&str_line, ',') == 0 {
            warn!("Does not contain expected comma character on line {}", line_num);
            skipped_lines.push(line_num);
            continue;
        }

        let columns: Vec<&str> = str_line.trim().split(',').collect();
        if columns.len() > 2 {
            let id = columns.get(0).unwrap().to_string();
            let url = columns.get(1).unwrap().to_string();
            projects.push(GitRepository {id:count.clone(), url});
            count = count + 1;
        } else {
            warn!("Err: Line {} is not formatted correctly and has been skipped.", line_num);
            skipped_lines.push(line_num);
        }
    }
    Ok(LinesResponse { response: projects, skipped_lines: None })
}

///Counts the number of matching characters in a String
fn character_count(str_line: &String, matching_character: char) -> u32 {
    let mut count: u32 = 0;

    for character in str_line.chars() {
        if character == matching_character {
            count += 1;
        }
    }
    return count;
}

pub fn clone_project(project: GitRepository) -> Result<ClonedProject, ErrorKind> {
    let home_path = get_home_dir_path().expect("Could not get home directory");
    let project_path = Path::new(&home_path)
        .join(String::from("project_analyser"))
        .join(String::from("repos"))
        .join(project.id.to_string());

    match check_url_http_code(&[200, 301], &project.url) { // TODO use constant for valid codes
        Ok(_) => {},
        Err(_) => { return Err(ErrorKind::NotFound)},
    }

    if !project_path.exists() {
        if fs::create_dir_all(&project_path).is_err() {
            warn!("Could not create project directory");
            return Err(ErrorKind::Other);
        };
    }


    let cloned_project = ClonedProject::new(project, project_path);

    info!("Downloading {} from {}", &cloned_project.github.id, &cloned_project.github.url);
    Command::new("git")
        .args(&["clone", &cloned_project.github.url, &cloned_project.path])
        .output()
        .expect("Could not clone project");
    info!("Downloaded {} from {}", &cloned_project.github.id, &cloned_project.github.url);

    Ok(cloned_project)
}

pub fn get_home_dir_path() -> Result<String, ErrorKind> {
    let home_dir = match env::home_dir() {
        None => PathBuf::from(""),
        Some(path) => PathBuf::from(path),
    };
    match home_dir.into_os_string().into_string() {
        Ok(s) => Ok(s),
        Err(_) => {
            error!("Could not convert home dir into string.");
            return Err(ErrorKind::Other); //("Could not convert home dir into string");
        }
    }
}

/// Checks whether the url exists using curl.
pub fn check_url_http_code(expected_codes: &[i32], url: &str) -> Result<(), ()> {
    // curl -s -o /dev/null-I  -I -w "%{http_code}"
    let curl = match Command::new("curl")
        .args(&["-s", "-o", "/dev/null", "-I", "-w", "\"%{http_code}\"", url])
        .output() {
        Ok(response) => response,
        Err(_) => { return Err(()); }
    };
    let http_code = utf8_to_http_code(curl.stdout)?;

    for expected_code in expected_codes {
        if http_code == *expected_code {
            return Ok(());
        }
    }
    warn!("Invalid http code for {}. Found {} .Valid codes are {:?}", url, http_code, expected_codes);
    Err(())
}


/// Tries to parse the specified data into a string and then into an integer
fn utf8_to_http_code(data: Vec<u8>) -> Result<i32, ()> {
    let code_string = match String::from_utf8(data) {
        Ok(code_string) => {
            code_string
        }
        Err(e) => {
            error!("Could not create string from curl's output. Treating url as not existent. Err: {}", e);
            return Err(());
        }
    };
    let stripped_code_string = code_string.replace('"', "");
    if stripped_code_string.len() != 3 {
        //Invalid HTTP response code
        error!("Invalid response code from curl");
        return Err(());
    }
    let result = match stripped_code_string.parse::<i32>() {
        Ok(code_i32) => code_i32,
        Err(e) => {
            error!("Could not parse http code '{}' into int. Treating url as not existent. Err: {}", code_string, e);
            return Err(());
        }
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn character_count_test_comma() {
        assert_eq!(character_count(&String::from(",,,,"), ','), 4);
    }

    #[test]
    fn character_count_test_number_as_param() {
        assert_eq!(character_count(&String::from(",,,,"), '4'), 0);
    }

    #[test]
    fn character_count_test_russian() {
        assert_eq!(character_count(&String::from("ру́сский язы́к"), 'й'), 1);
    }

    #[test]
    fn character_count_test_japanese() {
        assert_eq!(character_count(&String::from("日本語"), '語'), 1);
    }

    #[test]
    fn character_count_test_null_unicode() {
        assert_eq!(character_count(&String::from(""), '\0'), 0);
    }

    #[test]
    fn clone_project_correct_url() {
        let project = GitRepository { id: -1, url: String::from("https://github.com/bitcoin/bitcoin") };
        let success = clone_project(project).is_ok();
        let file_path = Path::new(&get_home_dir_path().unwrap())
            .join("project_analyser")
            .join("repos")
            .join("-1");
        if success {
            fs::remove_dir_all(file_path);
        }
        assert!(success);
    }

    #[test]
    fn clone_project_incorrect_url() {
        let project = GitRepository { id: 1, url: String::from("") };
        assert!(clone_project(project).is_err());
    }

    #[test]
    fn clone_project_incorrect_url_2() {
        let project = GitRepository { id: 1, url: String::from("https://github.com/private/private") };
        assert!(clone_project(project).is_err());
    }

    #[test]
    fn check_http_code_correct_url() {
        assert!(check_url_http_code(&[200], &String::from("https://github.com/bitcoin/bitcoin")).is_ok());
    }

    #[test]
    fn check_http_code_incorrect_url() {
        assert!(check_url_http_code(&[200], &String::from("https://github.com/private/private")).is_err());
    }

    #[test]
    fn check_http_code_incorrect_url_2() {
        assert!(check_url_http_code(&[200], &String::from("https://github.com/somewhere/over/the/rainbow")).is_err());
    }

    #[test]
    fn check_http_code_empty_url() {
        assert!(check_url_http_code(&[200], &String::from("")).is_err());
    }

    #[test]
    fn check_http_code_null_url() {
        assert!(check_url_http_code(&[200], &String::from("\0")).is_err());
    }

    #[test]
    fn check_http_code_garbage_url() {
        assert!(check_url_http_code(&[200], &String::from("23456£$%^23456\"")).is_err());
    }

    #[test]
    fn utf8_to_http_code_correct_code_404() {
        let mut input = vec![0x34, 0x30, 0x34];
        let result = match utf8_to_http_code(input) {
            Ok(result) => result,
            Err(_) => -1,
        };
        assert_eq!(result, 404);
    }

    #[test]
    fn utf8_to_http_code_correct_code_200() {
        let mut input = vec![0x032, 0x030, 0x030];
        let result = match utf8_to_http_code(input) {
            Ok(result) => result,
            Err(_) => -1,
        };
        assert_eq!(result, 200);
    }

    #[test]
    fn utf8_to_http_code_text() {
        let mut input = vec![0x60, 0x60, 0x60, 0x65, 0x65, 0x65, 0x23, 0x43];
        let result = match utf8_to_http_code(input) {
            Ok(result) => false,
            Err(_) => true,
        };
        assert!(result);
    }

    #[test]
    fn utf8_to_http_code_code_large() {
        let mut input = vec![0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32];
        let result = match utf8_to_http_code(input) {
            Ok(result) => false,
            Err(_) => true,
        };
        assert!(result);
    }

    //The following code tests functionality that will be changed soon.
    #[test]
    fn read_project_urls_should_pass() {
        let result = match read_project_urls_from_file(String::from("projects.csv")) {
            Ok(_) => { true },
            Err(_) => { false },
        };
        assert!(result);
    }
    #[test]
    #[should_panic]
    fn read_project_urls_should_panic() {
        read_project_urls_from_file(String::from("23453$^%£$dFGSf.csv"));
    }

    #[should_panic]
    fn read_project_urls_should_panic_2() {
        read_project_urls_from_file(String::from("23453$^%£$dFGSf.csv"));
    }
    //TODO: remove above tests

}











