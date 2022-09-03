use log::warn;
use regex::Regex;
use walkdir::WalkDir;

// use regex::Regex;
// let regex = Regex::new("issue: *NEXT-22338").unwrap();

pub fn find_matching_files(path: &str, regex: Regex) -> Vec<String> {
    let mut matching_files: Vec<String> = Vec::new();

    let directory_walker = WalkDir::new(path);
    for dir_entry in directory_walker.into_iter().filter_map(|e| e.ok()) {
        if !dir_entry
            .metadata()
            .expect("Couldn't read file metadata")
            .is_file()
        {
            continue;
        }

        let file_path = dir_entry.path();

        let file_content = match std::fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(error) => {
                log::warn!("Problem opening the file: {}", error.to_string());
                continue;
            }
        };

        if !regex.is_match(&file_content) {
            continue;
        }

        match file_path.to_str() {
            Some(path) => matching_files.push(path.to_owned()),
            None => warn!(
                "Path {} is not valid UTF-8",
                file_path.display()
            )
        }
    }

    return matching_files;
}