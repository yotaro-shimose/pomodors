use dirs;
use std::path::{Path, PathBuf};

pub fn get_pomodors_path() -> PathBuf {
    match dirs::home_dir() {
        Some(home) => home.join(".pomodors"),
        None => Path::new("/root").join(".pomodors"),
    }
}

pub fn get_conf_path() -> PathBuf {
    get_pomodors_path().join("config")
}

pub fn get_tokencache_path() -> PathBuf {
    get_pomodors_path().join("tokencache.json")
}

pub fn get_secret_path() -> PathBuf {
    get_pomodors_path().join("client_secret.json")
}
