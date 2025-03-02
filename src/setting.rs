use config::Config;
use serde::{Deserialize, Serialize};

use crate::dirs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub items: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AppSetting {
    pub groups: Vec<Group>,
    pub default_group: Option<String>,
}

impl AppSetting {
    pub fn get_instance() -> Self {
        let app_config_dir = dirs::get_app_config_dir();

        let config_file = app_config_dir.join("config.yaml");
        let config_file_path = config_file
            .to_str()
            .expect("Failed to resolve config file path");

        let app_config_builder = Config::builder()
            .add_source(config::File::with_name(config_file_path))
            .build()
            .expect("Failed to build config file");
        app_config_builder
            .try_deserialize::<AppSetting>()
            .expect("Failed to deserialize config file")
    }
}
