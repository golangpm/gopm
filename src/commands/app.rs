use std::{
    env,
    fs::{self, File},
    io::{Write},
    path::{Path},
};

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
pub fn create_new_go_app(app_name: &str) {
    // Создайте папку проекта с заданным именем
    fs::create_dir(app_name).expect("Failed to create the project directory");

    // Создайте структуру проекта и файлы
    let project_structure = crate::core::create_project_structure(app_name);

    for (entry, file_type) in &project_structure {
        let full_path = format!("{}/{}", app_name, entry);
        let cmd_app = format!("cmd/{app_name}/main.go");

        match file_type {
            crate::core::FileType::File => {
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

                println!("✔️ Created file: {}", entry);
            }
            crate::core::FileType::Directory => {
                fs::create_dir_all(&full_path).expect("Failed to create a directory");
            }
        }
    }

    crate::core::config::create_gpm_config_file(app_name).expect("Failed to create gpm-config.json");

    crate::commands::new_go_app_out(app_name);

}

pub fn init_go_app() {
    println!("Initializing the file structure in the current directory... 🚀");

    // Получите имя текущей директории и используйте его как имя вашего приложения
    let current_dir = env::current_dir().expect("❌ Failed to get the current directory");
    let app_name = current_dir.file_name().unwrap().to_str().unwrap();

    // Создайте структуру проекта и файлы
    let project_structure = crate::core::create_project_structure(app_name);

    for (entry, file_type) in &project_structure {
        let full_path = current_dir.join(entry);
        let cmd_app = format!("cmd/{app_name}/main.go");

        match file_type {
            crate::core::FileType::File => {
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
                        crate::commands::user::get_saved_author()
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

                println!("✔️ Created file: {}", entry);
            }
            crate::core::FileType::Directory => {
                fs::create_dir_all(&full_path).expect("❌ Failed to create a directory");
            }
        }
    }

    crate::commands::init_go_app_out(app_name);

}