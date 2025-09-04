use std::{fs::read_to_string, path::PathBuf};

use crate::{error::{AppError, AppResult}, AppArgs};


pub(crate) struct Config {
    pub(crate) cfg: String,
    pub(crate) dir: String,
}

impl TryFrom<&AppArgs> for Config {
    type Error = ();
    fn try_from(value: &AppArgs) -> Result<Self, Self::Error> {
        if let Some(config_path) = value.config.as_ref()
            && let Ok(valid_config_path) = config_path.canonicalize()
            && let Ok(config) = read_to_string(&valid_config_path)
            && let Some(parent) = valid_config_path.parent()
            && let Some(path) = parent.to_str()
        {
            return Ok(Config {
                cfg: config,
                dir: path.to_string()
            })
        }
        eprintln!("Invalid config path... Using default");
        Err(())
    }
}

impl Config {
    pub(crate) fn new() -> AppResult<Self> {
        let (config_dir, config_file_path) = Self::default_config_file_path()?;
        Ok(Config {
            cfg: read_to_string(&config_file_path)?,
            dir: config_dir,
        })
    }
    fn default_config_file_path() -> AppResult<(String, PathBuf)> {
        let home = std::env::home_dir().ok_or(AppError::HomeEnvNotSet)?;
        let config_dir_path = home.join(PathBuf::from(".config/crusty-keys"));
        let config_file_path = config_dir_path.join(PathBuf::from("config.lua"));

        // create a new config dir in $USER/.config
        // and generate a empty config.lua inside of it.
        if config_dir_path.canonicalize().ok().is_none() {
            let _ = std::fs::create_dir_all(&config_dir_path);
            let _ = std::fs::File::create_new(&config_file_path);
            println!("New config file created.")

        // if the $USER/.config exists but no config file is there we create a new one
        } else if config_file_path.canonicalize().ok().is_none() {
            let _ = std::fs::File::create_new(&config_file_path);
            println!("New config file created.")
        }

        let path_string = config_dir_path
            .to_str()
            .map(|s| s.to_string())
            .ok_or(AppError::ConfigCouldNotBeCreated)?;

        Ok((path_string, config_file_path))
    }
}


