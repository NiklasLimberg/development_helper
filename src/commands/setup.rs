use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Input};
use std::process;

#[path = "../tools/confy_wrapper.rs"]
mod confy_wrapper;

pub fn run() {
    println!("Welcome to the setup wizard");

    match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really really really really want to continue?")
        .interact_opt()
        .unwrap()
    {
        Some(true) => setup_wizard(),
        Some(false) => {
            println!("Exiting setup wizard");
            process::exit(exitcode::OK);
        }
        None => process::exit(exitcode::SOFTWARE),
    }
}

fn setup_wizard() {
    let mut config = confy_wrapper::get_config();

    config.author = Input::new()
        .with_prompt("Autor used in changelogs")
        .with_initial_text(config.author)
        .interact_text()
        .unwrap();

    config.author_email = Input::new()
        .with_prompt("Autor Email used in changelogs")
        .with_initial_text(config.author_email)
        .interact_text()
        .unwrap();

    config.author_github = Input::new()
        .with_prompt("Autor GitHub Account used in changelogs")
        .with_initial_text(config.author_github)
        .interact_text()
        .unwrap();

    if let Some(changelog_template) = Editor::new()
        .require_save(true)
        .edit(&config.changelog_template)
        .unwrap()
    {
        config.changelog_template = changelog_template;
    } else {
        println!("Abort!");
    }

    confy_wrapper::set_config(config)
}
