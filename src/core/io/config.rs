use std::{
    fs::{File},
    io,
    io::{Read, Write},
};

// Project .json file
pub fn read_project_file(file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let json: serde_json::Value = serde_json::from_str(&data)?;
    let res = json["name"].as_str().ok_or("Field 'name' not found")?;
    Ok(res.to_string())
}

// Create local GOPM project file
pub fn create_gpm_config_file(project_name: &str) -> io::Result<()> {
    let author = crate::commands::user::get_saved_author();

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
