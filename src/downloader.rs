use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::process::Command;
use std::str;
use std::io::ErrorKind;
use models::{GitRepository, ClonedProject, NewGitRepository};

pub struct LinesResponse <T> {
    pub response: Vec<T>,
    pub skipped_lines: Option<Vec<u32>>
}

/// Reads  the csv file "projects.csv" (see project root directory) and extracts the id and url for each row.
pub fn read_project_urls_from_file(filepath: String) -> Result<LinesResponse <NewGitRepository>, ErrorKind> {
    let csv_file = match File::open(filepath) {
        Ok(file) => file,
        Err(_) => {panic!("Could not open urls file")},
    };
    let reader = BufReader::new(csv_file);
    let mut projects: Vec<NewGitRepository> = Vec::new();
    let skip_rows = 1;
    let mut skipped_lines:Vec<u32> = Vec::new();
    let mut line_num:u32 = 1;
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
            let url = columns.get(1).unwrap().to_string();
            projects.push(NewGitRepository::new(url));
        } else {
            warn!("Err: Line {} is not formatted correctly and has been skipped.", line_num);
            skipped_lines.push(line_num);;
        }
    }
    Ok(LinesResponse { response: projects, skipped_lines:None})
}

///Counts the number of matching characters in a String
pub fn character_count(str_line: &String, matching_character: char) -> u32 {
    let mut count:u32 = 0;

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

    if !project_path.exists() {
        if fs::create_dir_all(&project_path).is_err() {
            warn!("Could not create project directory");
            return Err(ErrorKind::Other)
        };
    }

    let cloned_project = ClonedProject::new(project, project_path);
    if check_url_http_code(200, &cloned_project.github.url).is_err() {
        return Err(ErrorKind::Other)
    }

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
            return Err(ErrorKind::Other) //("Could not convert home dir into string");
        }
    }
}

/// Checks whether the url exists using curl.
pub fn check_url_http_code(expected_code: i32, url: &str) -> Result<(), ()> {
    // curl -s -o /dev/null-I  -I -w "%{http_code}"
    let curl = match Command::new("curl")
        .args(&["-s", "-o", "/dev/null", "-I", "-w", "\"%{http_code}\"", url])
        .output() {
            Ok(response) => response,
            Err(_) => {return Err(())},
        };

    let http_code = utf8_to_http_code(curl.stdout)?;
    println!("HTTP CODE {}", http_code);
    if http_code == expected_code {
        Ok(())
    } else {
        warn!("Http code does not match. Found {}, Expected {} for url {}", http_code, expected_code, url);
        Err(())
    }
}


/// Tries to parse the specified data into a string and then into an integer
pub fn utf8_to_http_code(data: Vec<u8>) -> Result<i32, ()> {
    let code_string = match String::from_utf8(data) {
        Ok(code_string) => {
            code_string
        },
        Err(e) => {
            error!("Could not create string from curl's output. Treating url as not existent. Err: {}", e);
            return Err(())
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
            return Err(())
        }
    };
    Ok(result)
}











