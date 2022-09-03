use std::process::{Command, Output};

pub fn create_branch (branch_name: String) {
    Command::new("git")
        .arg(&format!(
            "checkout -b {}",
            &branch_name
        ))
        .output()
        .expect("Could not create branch");

        println!("Created branch: {}", branch_name)

}

pub fn get_branch_name () -> Output {
    return Command::new("git")
        .arg("branch --show-current")
        .output()
        .expect("Could not read branch name");
        
}

pub fn create_commit (commit_msg: String) {
    Command::new("git")
        .arg(&format!(
            "commit -m {}",
            &commit_msg
        ))
        .status()
        .expect("Could not create commit");

        println!("Created commit: {}", commit_msg)
}