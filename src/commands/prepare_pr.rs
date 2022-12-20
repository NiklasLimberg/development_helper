use regex::Regex;
use std::{fs, path::Path, process};

use chrono::{self, Datelike};

use exitcode;

#[path = "../tools/confy_wrapper.rs"]
mod confy_wrapper;

#[path = "../tools/git_wrapper.rs"]
mod git_wrapper;

#[path = "../tools/open_editor.rs"]
mod editor;

pub fn run(issue_key: String, title: String) {
    let id_regex = Regex::new(r"NEXT-\d*").unwrap();

    if !id_regex.is_match(&issue_key) {
        println!(
            "Identifier '{}' does not match {}",
            issue_key,
            id_regex.as_str()
        );
        process::exit(exitcode::USAGE);
    };

    let formatted_title = trim_whitespace(&title).to_lowercase().replace(" ", "-");

    println!("Creating changelog");

    let changelog_path = get_filepath_for_changelog(&formatted_title);

    let mut write_changelog = true;

    if Path::new(&changelog_path).is_file() {
        println!(
            "A changelog with the path {} already exists!",
            changelog_path
        );
        match dialoguer::Confirm::new()
            .with_prompt("Do you want to overwrite the existing file?")
            .default(false)
            .show_default(true)
            .interact()
        {
            Ok(value) => write_changelog = value,
            Err(..) => {
                write_changelog = false;
                println!("Could not get answer keeping the current file")
            }
        }
    };

    if write_changelog {
        fs::write(&changelog_path, get_changelog_content(&title, &issue_key))
            .expect("Unable to write changelog file");

        editor::open(&changelog_path);

        git_wrapper::add_file(&changelog_path);
    }

    git_wrapper::create_branch(format!("{}/{}", issue_key.to_lowercase(), formatted_title));

    if git_wrapper::get_staged_file_names().len() == 0 {
        println!("Skipping creating a commit. No staged files to commit");
    } else {
        git_wrapper::create_commit(format!("{} - {}", issue_key, title));
    }
}

fn get_filepath_for_changelog(formatted_title: &String) -> String {
    let current_time = chrono::Local::now();

    return format!(
        "./changelog/_unreleased/{}-{}-{}-{}.md",
        current_time.year(),
        current_time.month(),
        current_time.day(),
        formatted_title
    );
}

fn get_changelog_content(title: &String, issue_key: &String) -> String {
    let config = confy_wrapper::get_config();

    return config
        .changelog_template
        .replace("${title}", &title)
        .replace("${issue}", &issue_key)
        .replace("${author}", &config.author)
        .replace("${author_email}", &config.author_email)
        .replace("${author_github}", &config.author_github);
}

// copied from https://stackoverflow.com/a/71864244
fn trim_whitespace(s: &str) -> String {
    let mut new_str = s.trim().to_owned();
    let mut prev = ' '; // The initial value doesn't really matter
    new_str.retain(|ch| {
        let result = ch != ' ' || prev != ' ';
        prev = ch;
        result
    });
    new_str
}
