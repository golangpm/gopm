///  The build_and_run_go_app() function
/// This function builds & run golang application from zero
/// Command to build: go build -o bin/{app_name} cmd/{app_name}/main.go
/// Command to run: ./bin/{app_name}
/// Starting the binary file...
pub fn build_and_run_go_app() {
    println!("📦⚙️Building and running the Go application...");
    let app_name = match crate::core::config::read_project_file("gpm-config.json") {
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
        .spawn();
    }

/// The build_go_app() function
/// This functions builds golang application at the `bin/` dir
/// Command: go build -o bin/{app_name} cmd/{app_name}/main.go
pub fn build_go_app() {
    println!("📦 Building the Go application...");

    let app_name = match crate::core::config::read_project_file("gpm-config.json") {
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