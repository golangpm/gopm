use std::{
    fs::{self, File},
    io::{BufReader, Write},
    path::{Path},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    author: Option<String>,
}

pub fn get_saved_author() -> String {
    let config_file_path = match std::env::consts::OS {
        "windows" => crate::consts::WINDOWS_CONFIG_PATH.to_str().unwrap(),
        "macos" => crate::consts::MACOS_CONFIG_PATH.to_str().unwrap(),
        "linux" => crate::consts::LINUX_CONFIG_PATH.to_str().unwrap(),
        _ => {
            println!("Unsupported operating system");
            return String::new();
        }
    };

    if !Path::new(config_file_path).exists() {
        println!("No configuration file found.");
        return String::new();
    }

    let file = match File::open(config_file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open the configuration file");
            return String::new();
        }
    };

    let reader = BufReader::new(file);

    let config: Config = match serde_json::from_reader(reader) {
        Ok(config) => config,
        Err(_) => {
            println!("Failed to read the configuration file");
            return String::new();
        }
    };

    if let Some(author) = config.author {
        author
    } else {
        println!("No saved author found.");
        String::new()
    }
}

pub fn print_username() {
    if get_saved_author().is_empty() {
        println!("Please, set your username by command:\n\n\tgopm set-author <username>");
        return;
    }
    println!("Your username {}", get_saved_author());
}

pub fn save_author_to_config(author: &str) {
    let config_file_name = "gopm-config.json";
    let config_dir = match dirs::home_dir() {
        Some(mut dir) => {
            dir.push("gopm");
            dir.push("config");
            dir
        }
        None => {
            println!("Failed to determine home directory.");
            return;
        }
    };

    if !config_dir.exists() {
        if let Err(_) = fs::create_dir_all(&config_dir) {
            println!("Failed to create the configuration directory");
            return;
        }
    }

    let config_file_path = config_dir.join(config_file_name);

    let config = Config {
        author: Some(author.to_string()),
    };

    let config_json = match serde_json::to_string_pretty(&config) {
        Ok(json) => json,
        Err(_) => {
            println!("Failed to serialize the configuration");
            return;
        }
    };

    let mut file = match File::create(&config_file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to create the configuration file");
            return;
        }
    };

    if let Err(_) = file.write_all(config_json.as_bytes()) {
        println!("Failed to write to the configuration file");
    } else {
        println!("Author saved successfully!");
    }
}