extern crate confy;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub changelog_template: String,
    pub author: String,
    pub author_email: String,
    pub author_github: String,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            changelog_template: include_str!("../defaults/default_changelog.txt").to_string(),
            author: "".to_string(),
            author_email: "".to_string(),
            author_github: "".to_string(),
        }
    }
}

pub fn get_config() -> AppConfig {
    return confy::load("development-helper").expect("Is not able to read/create config");
}

pub fn set_config(new_config: AppConfig) {
    return confy::store("development-helper", new_config).expect("Could not store config");
}
