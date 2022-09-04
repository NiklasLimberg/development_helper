use std::process::{Command, Output};

pub fn create_branch(branch_name: String) {
    Command::new("git")
        .args(["checkout", "-b", &branch_name])
        .status()
        .expect("Could not create branch");

    println!("Created branch: {}", branch_name)
}

pub fn get_branch_name() -> Output {
    return Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("Could not read branch name");
}

pub fn get_staged_file_names() -> Vec<String> {
    let output = Command::new("git")
        .args(["diff", "--name-only", "--staged"])
        .output()
        .expect("Could not read staged file names");

    let mut lines: Vec<String> = Vec::new();

    let output_string = String::from_utf8(output.stdout)
        .expect("Could not parse git output as utf-8")
        .to_string();

    for line in output_string.lines() {
        lines.push(line.to_string())
    }

    return lines;
}

pub fn create_commit(commit_msg: String) {
    Command::new("git")
        .args(["commit", "-m", &commit_msg])
        .status()
        .expect("Could not create commit");

    println!("Created commit: {}", commit_msg)
}
