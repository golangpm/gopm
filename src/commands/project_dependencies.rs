use regex::Regex;
use std::{
    fs::{File},
    io,
    io::{BufRead, BufReader},

};

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