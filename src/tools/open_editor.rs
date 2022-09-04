use std::{env::var, process::Command};

// todo parse editor from config
pub fn open(file_path: &str) {
    let default_editor = var("EDITOR").unwrap_or("nano".to_string());

    Command::new(&default_editor)
        .arg(file_path)
        .status()
        .expect(&format!(
            "Could not open file {} in {}",
            &file_path, default_editor
        ));
}
