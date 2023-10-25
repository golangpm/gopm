/// The FileType enumeration was created to control file's type, for example:
/// We have to create some this file structure like this one:
/// ```
/// .
/// |-- Makefile
/// |-- README.md
/// |-- bin
/// |-- cmd
/// |   `-- crsdd
/// |       `-- main.go
/// |-- Scripts
/// |-- docs
/// |   |-- docs.md
/// |   `-- src
/// |       `-- imgs
/// ```
///
/// By FileType enum we can manage, what type of file we will create - file or a folder
/// Usage example:
///
/// We have to create a Vector, that will keeps our values (files, folders)
/// ```
///  let mut project_structure = Vec::new();
///  project_structure.push(("Makefile".to_string(), FileType::File));
///  project_structure.push(("README.md".to_string(), FileType::File));
///  project_structure.push(("Scripts/install.sh".to_string(), FileType::File));
///  project_structure.push(("bin".to_string(), FileType::Directory));
/// // we can create a vector element with formant value
///
/// let cmd_go_file = format!("cmd/{}/main.go", app_name);
/// project_structure.push((cmd_go_file, FileType::File));
///
/// // After, we return our Vector
/// project_structure
/// ```

/// FileType Enumerate types:
/// File - we use this type to create a file
/// Directory - we use this type to create a folder
pub enum FileType {
    File,
    Directory,
}

/// The `create_project_structure` function takes an `app_name` &str and returns Vec of Strings and Filetype struct
/// We creating a files structure by this function!
/// For example, the default golang file arch:
///
/// ```
/// .
/// |-- Makefile
/// |-- README.md
/// |-- Scripts
/// |   `-- install.sh
/// |-- bin
/// |-- cmd
/// |   `-- crsdd
/// |       `-- main.go
/// |-- configs
/// |-- docs
/// |   |-- docs.md
/// |   `-- src
/// |       `-- imgs
/// |-- gpm-config.json
/// `-- pkg
///     |-- consts
///     |   `-- consts.go
///     `-- utils
///         `-- utils.go
/// ```
pub fn create_project_structure(app_name: &str) -> Vec<(String, FileType)> {
    let mut project_structure = Vec::new();

    project_structure.push(("Makefile".to_string(), FileType::File));
    project_structure.push(("README.md".to_string(), FileType::File));
    project_structure.push(("Scripts/install.sh".to_string(), FileType::File));
    project_structure.push(("bin".to_string(), FileType::Directory));

    let cmd_go_file = format!("cmd/{}/main.go", app_name);
    project_structure.push((cmd_go_file, FileType::File));

    project_structure.push(("configs".to_string(), FileType::Directory));
    project_structure.push(("docs/docs.md".to_string(), FileType::File));
    project_structure.push(("docs/src/imgs".to_string(), FileType::Directory));
    project_structure.push(("gpm-config.json".to_string(), FileType::File));
    project_structure.push(("pkg/consts/consts.go".to_string(), FileType::File));
    project_structure.push(("pkg/utils/utils.go".to_string(), FileType::File));

    project_structure
}
