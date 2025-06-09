use std::{fs, path::PathBuf};

use inquire::{Confirm, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub projects_path: String,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        Self {
            projects_path: home_dir.join("code").to_string_lossy().to_string(),
        }
    }
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    match fs::read_to_string(&config_path) {
        Ok(content) => {
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        }
        Err(_) => {
            if let Some(config_parent_path) = config_path.parent() {
                fs::create_dir_all(config_parent_path)?;
            }
            create_config_interactive(config_path)
        }
    }
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    Ok(home_dir.join(".config/stevio_code/config.toml"))
}

fn create_config_interactive(config_path: PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    println!("Looks like this is your first time using StevioCode!");
    println!("Let's set up your configuration...\n");

    let default_project_path = dirs::home_dir()
        .unwrap()
        .join("code")
        .to_string_lossy()
        .to_string();
    let projects_path = Text::new("Where do you keep your projects?")
        .with_default(&default_project_path)
        .with_help_message(
            "This is the root directory where SteVioCode will scan for Git repositories.",
        )
        .prompt()?;
    let config = Config { projects_path };
    println!("\n Configuration Preview:");
    println!("─────────────────────────");
    println!("Projects Path: {}", config.projects_path);

    let confirm_save = Confirm::new("\nSave this configuration?")
        .with_default(true)
        .prompt()?;

    if confirm_save {
        let toml_config = toml::to_string_pretty(&config)?;
        fs::write(&config_path, &toml_config)?;
        Ok(config)
    } else {
        println!("X Configuration not saved. Using defaults for this session.");
        Ok(Config::default())
    }
}
