use regex::{Regex, RegexBuilder};
use std::{env, fs, path::Path, process};

use exitcode;

#[path = "../tools/git_wrapper.rs"]
mod git_wrapper;

#[path = "../tools/find_matching_files.rs"]
mod matching_files;

#[path = "../tools/open_editor.rs"]
mod editor;

fn get_ticket_number_from_branch_name() -> String {
    let output = String::from_utf8(git_wrapper::get_branch_name().stdout)
        .expect("Could not convert branch name to UTF-8");

    let branch_name = output.trim();

    print!("Currently on branch '{}' ", branch_name);

    if branch_name.trim() == "trunk" {
        print!("Cannot find changelogs on the 'main' branch");
        process::exit(exitcode::CONFIG);
    }

    let ticket_regex = Regex::new(r"(next-\d+)").unwrap();

    return ticket_regex
        .find(&branch_name)
        .expect("Could not parse ticket from current branch name")
        .as_str()
        .into();
}

pub fn run(id: Option<String>) {
    println!("Trying to find ticket number");

    let ticket_number = match id {
        Some(id) => id,
        None => get_ticket_number_from_branch_name(),
    };

    let file_content_regex = RegexBuilder::new(&ticket_number)
        .case_insensitive(true)
        .build()
        .expect("Could not create regex from ticket number");

    let changelog_dir = Path::new("./changelog");

    if !changelog_dir.is_dir() {
        print!(
            "Could not find changelog directory in current working directory with path {}",
            changelog_dir.display()
        );
        process::exit(exitcode::USAGE);
    }

    let absolute_changelog_path = changelog_dir
        .canonicalize()
        .expect("Could not resolve changelog directory in the current working directory");

    let absolute_changelog_path_string = absolute_changelog_path
        .to_str()
        .expect("Could not turn absolute changelog directory path into UTF-8");

    let matches =
        matching_files::find_matching_files(absolute_changelog_path_string, file_content_regex);

    if matches.len() == 0 {
        println!(
            "Couldn't find ticket number {} in any files in {}",
            ticket_number, absolute_changelog_path_string
        )
    }

    if matches.len() == 1 {
        editor::open(&matches[0])
    }

    if matches.len() > 1 {
        let selected_indices = dialoguer::MultiSelect::new()
            .items(&matches)
            .with_prompt(format!(
                "Found multiple files with {} in them",
                ticket_number
            ))
            .interact()
            .expect("Could not read selected options");

        for indices in selected_indices {
            editor::open(&matches[indices])
        }
    }
}
