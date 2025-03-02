use directories::BaseDirs;

fn get_base_dirs() -> directories::BaseDirs {
    BaseDirs::new().expect("Could not determine user directories")
}

pub fn get_app_config_dir() -> std::path::PathBuf {
    get_base_dirs().config_local_dir().join("random-picker")
}

pub fn get_app_cache_dir() -> std::path::PathBuf {
    get_base_dirs().cache_dir().join("random-picker")
}
