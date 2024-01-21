use std::{path::{PathBuf, Path}, io::Write};

use directories::ProjectDirs;

use crate::APP_NAME;

#[derive(Debug)]
pub struct ConfigError {
    pub path: String 
}

pub fn get_keymaps() -> Result<PathBuf, ConfigError> {
    let directories = ProjectDirs::from("dev", "zenqrt", APP_NAME).expect("Failed to find/create project directory");
    let config_dir = directories.config_dir();

    if !config_dir.exists() {
        std::fs::create_dir_all(config_dir).expect("Failed to create config directory");
    }

    let keymaps_path = directories.config_dir().join(Path::new("keybinds.json"));
    
    if !keymaps_path.exists() {
        let mut file = std::fs::File::create(keymaps_path.clone()).expect("Failed to create keymaps file");
        file.write_all(b"[]").expect("Failed to write in keymaps file");
    }

    Ok(keymaps_path)
}
