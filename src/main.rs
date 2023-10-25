mod core;

use clap::{App, Arg, SubCommand};
use once_cell::sync::Lazy;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use regex::Regex;
use std::{
    io,
    env,
    io::{BufReader, Write, BufRead},
};

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    author: Option<String>,
}

static WINDOWS_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::home_dir().unwrap_or_else(|| {
        println!("Failed to get the home directory");
        std::process::exit(1);
    });
    path.push("gopm/config/gopm-config.json");
    path
});

// Local Consts
/// DEBUG режим - Используется для отлажки программы:
/// При активном DEBUG режиме все файлы создаются в директории `/out/...` - это сделано для того,
/// чтобы не засорять основную директорию проекта. При билде программы для продакшн режима рекомендуется установить значение данной переменной в false
const DEBUG: bool = true;
static MACOS_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::home_dir().unwrap_or_else(|| {
        println!("Failed to get the home directory");
        std::process::exit(1);
    });
    path.push("gopm/config/gopm-config.json");
    path
});
static LINUX_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::home_dir().unwrap_or_else(|| {
        println!("Failed to get the home directory");
        std::process::exit(1);
    });
    path.push("gopm/config/gopm-config.json");
    path
});

fn main() {
    let matches = App::new("gopm")
        .version("1.0")
        .author("Your Name")
        .about("A Go project manager and template generator")
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new Go application with a custom file structure template")
                .arg(Arg::with_name("app_name").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize the file structure of the current directory"),
        )
        .subcommand(SubCommand::with_name("run").about("Build and run the Go application"))
        .subcommand(SubCommand::with_name("build").about("Build the Go application"))
        .subcommand(SubCommand::with_name("get-author").about("Get the saved author"))
        .subcommand(
            SubCommand::with_name("set-author")
                .about("Save the author to the configuration")
                .arg(Arg::with_name("author").required(true).index(1)),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("new") => {
            let sub_matches = matches.subcommand_matches("new").unwrap();
            let app_name = sub_matches.value_of("app_name").unwrap();
            create_new_go_app(app_name);
        }
        Some("init") => init_go_app(),
        Some("run") => build_and_run_go_app(),
        Some("build") => build_go_app(),
        Some("get-author") => print_username(),
        Some("set-author") => {
            let sub_matches = matches.subcommand_matches("set-author").unwrap();
            let author = sub_matches.value_of("author").unwrap();
            save_author_to_config(author);
        }
        _ => eprintln!("Invalid command. Use 'gopm --help' for usage information."),
    }
}

/// Enumerate, содержащий параметры: File и Directory...
///  Используется дл ясоздаения файловой структуры проекта.
/// Это поможет чётко обозначить в шаблоне, какой компонент является файлом, а какой директорией
/// ```
/// let project_structure = [
///    ("README.md", FileType::File),
///    ("Scripts/install.sh", FileType::File),
///    ("bin", FileType::Directory),
///    (&format!("cmd/{}.go", app_name), FileType::File),
///    ("docs/docs.md", FileType::File),
///    ("gpm-config.json", FileType::File),
///    ("pkg/consts/consts.go", FileType::File),
///];
/// ```
fn create_new_go_app(app_name: &str) {
    let project_structure = core::create_project_structure(app_name);

    let root_dir = if DEBUG {
        app_name.to_string()
    } else {
        format!("out/{}", app_name)
    };

    fs::create_dir_all(&root_dir).expect("Failed to create the project directory");

    for (entry, file_type) in &project_structure {
        let full_path = format!("{}/{}", root_dir, entry);

        match file_type {
            core::FileType::File => {
                if let Some(parent_dir) = Path::new(&full_path).parent() {
                    fs::create_dir_all(parent_dir).expect("Failed to create a directory");
                }

                // Check if the file is the Makefile
                if entry == &"Makefile" {
                    let makefile_content = format!(
                        r#"# --- Variables ---
appname = {app_name}

# --- Actions ---
Default:
	go run cmd/$(appname).go

# --- Build an example ---
# Make "examples" folder (Unix)
example:
	cp -rv `ls -A | grep -vE ".git|.env|.gitignore|.vscode|.idea|.Ds_Store|README.md|examples|test"` examples
# Make "examples" folder (Windows)
example-win:
	robocopy "." "examples" /xf ".gitignore" ".env" "README.md" /xd ".git" ".Ds_Store" ".vscode" ".idea" "assets" "test" "examples" /s
	echo -e "examples" folder was builded!

"#);

                    // Create the Makefile and write the content
                    let mut makefile =
                        File::create(&full_path).expect("Failed to create the Makefile");
                    makefile
                        .write_all(makefile_content.as_bytes())
                        .expect("Failed to write to the Makefile");
                } else {
                    // Create other files as usual
                    File::create(&full_path).expect("Failed to create a file");
                }
            }
            core::FileType::Directory => {
                fs::create_dir_all(&full_path).expect("Failed to create a directory");
            }
        }
    }

    // Generate the content for the main Go source file
    let app_file_path = format!("{}/cmd/{}.go", root_dir, app_name);
    let app_source_code = format!(
        r#"package main

import "fmt"

func main() {{
    fmt.Println("Hello, {}!")
}}
"#,
        app_name
    );

    // Write the generated content to the main Go source file
    let mut app_file = File::create(&app_file_path)
        .expect("Failed to create the main Go source file");
    app_file
        .write_all(app_source_code.as_bytes())
        .expect("Failed to write to the main Go source file");

    if let Err(err) = create_gpm_config_file(app_name, "go.mod") {
        eprintln!("Error: {}", err);
    }
}

// Init Go Ppp
fn init_go_app() {
    println!("Initializing the file structure of the current directory... 🚀");
    // Получаем текущую рабочую директорию
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Извлекаем имя папки из полного пути
    let app_name = current_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    // Создаем проектную структуру
    let project_structure = core::create_project_structure(app_name);

    // Извлекаем имя папки из полного пути
    let app_name = current_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();


    for (path, file_type) in project_structure.iter() {
        match file_type {
            core::FileType::File => {
                let file_path = current_dir.join(path);
                core::create_file(file_path.to_str().unwrap());
                println!("Created file: {}", path);
            }
            core::FileType::Directory => {
                let dir_path = current_dir.join(path);
                core::create_directory(dir_path.to_str().unwrap());
                println!("Created directory: {}", path);
            }
        }
    }

    println!("Go project initialized successfully!");
}

fn build_and_run_go_app() {
    println!("Building and running the Go application... 🛠️🏃");
}

fn build_go_app() {
    println!("Building the Go application... 🛠️");
}

fn get_saved_author() -> String {
    let config_file_path = match std::env::consts::OS {
        "windows" => WINDOWS_CONFIG_PATH.to_str().unwrap(),
        "macos" => MACOS_CONFIG_PATH.to_str().unwrap(),
        "linux" => LINUX_CONFIG_PATH.to_str().unwrap(),
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

fn print_username() {
    println!("Your username {}", get_saved_author());
}

fn save_author_to_config(author: &str) {
    let config_file_name = "gopm-config.json";
    let config_dir = match dirs::home_dir() {
        Some(mut dir) => {
            dir.push("gopm");
            dir.push("config");
            dir
        },
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

// Project .json file

// Parse projec dependencies
fn parse_dependencies_from_go_mod(go_mod_path: &str) -> io::Result<Vec<String>> {
    let mut dependencies = Vec::new();

    let go_mod_file = File::open(go_mod_path)?;
    let reader = BufReader::new(go_mod_file);

    let re_dependency = Regex::new(r#"^\s*(\S+)\s+v\d+\.\d+\.\d+.*$"#)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?; // Преобразование ошибки

    for line in reader.lines() {
        let line = line?;
        if let Some(captures) = re_dependency.captures(&line) {
            let dependency = captures.get(1).expect("Failed to parse dependency").as_str();
            dependencies.push(dependency.to_string());
        }
    }

    Ok(dependencies)
}


// Create local GOPM project file
fn create_gpm_config_file(project_name: &str, go_mod_path: &str) -> io::Result<()> {
    let author = get_saved_author();
    let dependencies = parse_dependencies_from_go_mod(go_mod_path)?;

    let mut file_content = format!(
        r#"{{
    "name": "{}",
    "author": "{}",
    "version": "0.0.0",
    "scripts": {{
        "run": "gopm run",
        "start": "gopm start",
        "build": "gopm build"
    }},
    "dependencies": {{"#,
        project_name, author
    );

    for dependency in dependencies {
        file_content.push_str(format!(r#"
        "{}": "latest","#, dependency).as_str());
    }

    file_content.push_str("\n    }\n}");

    let file_path = format!("{}/gpm-config.json", project_name);

    let mut file = File::create(&file_path)?;
    file.write_all(file_content.as_bytes())?;

    Ok(())
}
