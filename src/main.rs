mod core;

use clap::{App, Arg, SubCommand};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    env,
    fs::{self, File},
    io,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
};

extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    author: Option<String>,
}

// CONSTS
const VERSION: &str = "0.0.3";
const AUTHOR: &str = "ITDOBRO";

static WINDOWS_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::home_dir().unwrap_or_else(|| {
        println!("❌ Failed to get the home directory");
        std::process::exit(1);
    });
    path.push("gopm/config/gopm-config.json");
    path
});

static MACOS_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::home_dir().unwrap_or_else(|| {
        println!("❌ Failed to get the home directory");
        std::process::exit(1);
    });
    path.push("gopm/config/gopm-config.json");
    path
});
static LINUX_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::home_dir().unwrap_or_else(|| {
        println!("❌ Failed to get the home directory");
        std::process::exit(1);
    });
    path.push("gopm/config/gopm-config.json");
    path
});

fn main() {
    let matches = App::new("gopm")
        .version(VERSION)
        .author(AUTHOR)
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
                .about("Save the author to the configuration\n\tExample: set-author Your Name")
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
        _ => core::utils::logo(),
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
    // Создайте папку проекта с заданным именем
    fs::create_dir(app_name).expect("Failed to create the project directory");

    // Создайте структуру проекта и файлы
    let project_structure = core::create_project_structure(app_name);

    for (entry, file_type) in &project_structure {
        let full_path = format!("{}/{}", app_name, entry);
        let cmd_app = format!("cmd/{app_name}/main.go");

        match file_type {
            core::FileType::File => {
                if let Some(parent_dir) = Path::new(&full_path).parent() {
                    fs::create_dir_all(parent_dir).expect("Failed to create a directory");
                }

                let file_content = match entry.as_str() {
                    "Makefile" => {
                        format!(
                            r#"# --- Variables ---
appname = {}

# --- Actions ---
Default:
	go run cmd/$(appname)/main.go
start:
	./bin/$(appname)
build:
	go build -o bin/$(appname) cmd/$(appname)/main.go

# --- Build an example ---
# Make "examples" folder (Unix)
example:
	cp -rv `ls -A | grep -vE ".git|.env|.gitignore|.vscode|.idea|.Ds_Store|README.md|examples|test"` examples
# Make "examples" folder (Windows)
example-win:
	robocopy "." "examples" /xf ".gitignore" ".env" "README.md" /xd ".git" ".Ds_Store" ".vscode" ".idea" "assets" "test" "examples" /s
	echo -e "examples" folder was built!

"#,
                            app_name
                        )
                    }
                    _ => {
                        if entry == &cmd_app {
                            format!(
                                r#"package main

import "fmt"

func main() {{
    fmt.Println("Hello, {}!")
}}
"#,
                                app_name
                            )
                        } else {
                            "".to_string()
                        }
                    }
                };

                let mut file = File::create(&full_path).expect("Failed to create a file");
                file.write_all(file_content.as_bytes())
                    .expect("Failed to write to the file");

                println!("Created file: {}", entry);
            }
            core::FileType::Directory => {
                fs::create_dir_all(&full_path).expect("Failed to create a directory");
            }
        }
    }

    // Создайте файл gpm-config.json
    create_gpm_config_file(app_name).expect("Failed to create gpm-config.json");

    println!("Go project created successfully!");
}

fn init_go_app() {
    println!("Initializing the file structure in the current directory... 🚀");

    // Получите имя текущей директории и используйте его как имя вашего приложения
    let current_dir = env::current_dir().expect("Failed to get the current directory");
    let app_name = current_dir.file_name().unwrap().to_str().unwrap();

    // Создайте структуру проекта и файлы
    let project_structure = core::create_project_structure(app_name);

    for (entry, file_type) in &project_structure {
        let full_path = current_dir.join(entry);
        let cmd_app = format!("cmd/{app_name}/main.go");

        match file_type {
            core::FileType::File => {
                if let Some(parent_dir) = full_path.parent() {
                    fs::create_dir_all(parent_dir).expect("Failed to create a directory");
                }

                let file_content = match entry.as_str() {
                    "Makefile" => {
                        format!(
                            "# --- Variables ---\nappname = {}\n\n# --- Actions ---\nDefault:\n\tgo run cmd/$(appname)/main.go\nstart:\n\t./bin/$(appname)\nbuild:\n\tgo build -o bin/$(appname) cmd/$(appname)/main.go\n\n# --- Build an example ---\n# Make \"examples\" folder (Unix)\nexample:\n\tcp -rv `ls -A | grep -vE \".git|.env|.gitignore|.vscode|.idea|.Ds_Store|README.md|examples|test\"` examples\n# Make \"examples\" folder (Windows)\nexample-win:\n\trobocopy \".\" \"examples\" /xf \".gitignore\" \".env\" \"README.md\" /xd \".git\" \".Ds_Store\" \".vscode\" \".idea\" \"assets\" \"test\" \"examples\" /s\necho -e \"examples\" folder was built!\n",
                            app_name
                        )
                    }
                    "gpm-config.json" => format!(
                        r#"{{
    "name": "{}",
    "author": "{}",
    "version": "0.0.0",
    "scripts": {{
        "run": "gopm run",
        "start": "gopm start",
        "build": "gopm build"
    }}
}}"#,
                        app_name,
                        get_saved_author()
                    ),
                    _ => {
                        if entry == &cmd_app {
                            format!(
                                r#"package main

import "fmt"

func main() {{
    fmt.Println("Hello, {}!")
}}
"#,
                                app_name
                            )
                        } else {
                            "".to_string()
                        }
                    }
                };

                let mut file = File::create(full_path).expect("Failed to create a file");
                file.write_all(file_content.as_bytes())
                    .expect("❌ Failed to write to the file");

                println!("📄 Created file: {}", entry);
            }
            core::FileType::Directory => {
                fs::create_dir_all(&full_path).expect("❌ Failed to create a directory");
            }
        }
    }
}

fn build_and_run_go_app() {
    println!("📦⚙️Building and running the Go application...");
    let app_name = match read_project_file("gpm-config.json") {
        Ok(name) => name,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    let _build_cmd = std::process::Command::new("go")
        .arg("build")
        .arg("-o")
        .arg(format!("bin/{}", app_name))
        .arg(format!("cmd/{}/main.go", app_name))
        .spawn();

    println!("✅ Build successful at the `bin/{}`\n🏃 Runnung...\n\n", app_name);

    let _run_cmd = std::process::Command::new(format!("./bin/{app_name}"))
        .arg("build")
        .arg("-o")
        .arg(format!("bin/{}", app_name))
        .arg(format!("cmd/{}/main.go", app_name))
        .spawn();
    }

fn read_project_file(file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let json: serde_json::Value = serde_json::from_str(&data)?;
    let res = json["name"].as_str().ok_or("Field 'name' not found")?;
    Ok(res.to_string())
}

fn build_go_app() {
    println!("📦 Building the Go application...");

    let app_name = match read_project_file("gpm-config.json") {
        Ok(name) => name,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    let build_cmd = std::process::Command::new("go")
        .arg("build")
        .arg("-o")
        .arg(format!("bin/{}", app_name))
        .arg(format!("cmd/{}/main.go", app_name))
        .spawn();

    match build_cmd {
        Ok(mut child) => {
            let status = child.wait();
            match status {
                Ok(exit_status) => {
                    if exit_status.success() {
                        println!("✅ Build successful at the `bin/{}`", app_name);
                    } else {
                        eprintln!("❌ Build failed.");
                    }
                }
                Err(err) => {
                    eprintln!("⚠️ Failed to wait for build process: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("⚠️ Failed to start build process: {}", err);
        }
    }
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

// Project .json file

// Parse project dependencies
#[allow(dead_code)]
fn parse_dependencies_from_go_mod(go_mod_path: &str) -> io::Result<Vec<String>> {
    let mut dependencies = Vec::new();

    let go_mod_file = File::open(go_mod_path)?;
    let reader = BufReader::new(go_mod_file);

    let re_dependency = Regex::new(r#"^\s*(\S+)\s+v\d+\.\d+\.\d+.*$"#)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?; // Преобразование ошибки

    for line in reader.lines() {
        let line = line?;
        if let Some(captures) = re_dependency.captures(&line) {
            let dependency = captures
                .get(1)
                .expect("Failed to parse dependency")
                .as_str();
            dependencies.push(dependency.to_string());
        }
    }

    Ok(dependencies)
}

// Create local GOPM project file
fn create_gpm_config_file(project_name: &str) -> io::Result<()> {
    let author = get_saved_author();

    let mut file_content = format!(
        r#"{{
    "name": "{}",
    "author": "{}",
    "version": "0.0.0",
    "scripts": {{
        "run": "gopm run",
        "start": "gopm start",
        "build": "gopm build"
    }}"#,
        project_name, author
    );

    file_content.push_str("\n}");

    let file_path = format!("{}/gpm-config.json", project_name);

    let mut file = File::create(&file_path)?;
    file.write_all(file_content.as_bytes())?;

    Ok(())
}
