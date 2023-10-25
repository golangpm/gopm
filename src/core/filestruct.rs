pub enum FileType {
    File,
    Directory,
}

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
