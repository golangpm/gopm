use crate::commands::user::*;

pub fn new_go_app_out(app_name: &str) {
    let author = get_saved_author();

    if get_saved_author().is_empty() {
        println!("🚀 Go project created successfully!\n\n\
        Use this commands to start your app:\n
           cd {app_name}\n
           go mod init github.com/{app_name}\n
           go mod tidy\n
           gopm run");
    } else {
        println!("🚀 Go project created successfully!");
        let output = format!(
            "\n\n🧪 Use this commands to start your app:
            \n\n   cd {app_name}
            \n   go mod init github.com/{author}/{app_name}
            \n   go mod tidy
            \n   gopm run");
        println!("{output}");
    }
}

pub fn init_go_app_out(app_name: &str) {
    let author = get_saved_author();

    if get_saved_author().is_empty() {
        println!("🚀 Go project created successfully!\n\n
        Use this commands to start your app:\n
           go mod init github.com/{app_name}\n\
           go mod tidy\n\
           gopm run");
    } else {
        println!("🚀 Go project created successfully!");
        let output = format!(
            "\n\n🧪 Use this commands to start your app:\
            \n   go mod init github.com/{author}/{app_name}\
            \n   go mod tidy\
            \n   gopm run");
        println!("{output}");
    }
}