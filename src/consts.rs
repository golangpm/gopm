/// The Gloabal Constants Module
/// Here is the place to keep constants
/// APP {VERSION, AUTHOR}
/// PATHS {WINDOWS/LINUX/MACOS}PATHS

use once_cell::sync::Lazy;
use std::path::PathBuf;

// CONSTS
/// App VERSION
pub const VERSION: &str = "0.0.4";
/// App's Author
pub const AUTHOR: &str = "ITDOBRO";

/// Path to the Windows config
/// File: gopm-config.json
/// PATH: C:\Users\<$USER>\gopm\config\gopm-config.json
pub static WINDOWS_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::home_dir().unwrap_or_else(|| {
        println!("❌ Failed to get the home directory");
        std::process::exit(1);
    });
    path.push("gopm/config/gopm-config.json");
    path
});

/// 🐧 Unix PATHS

/// Path to the Mac OS config
/// File: gopm-config.json
/// PATH: ~/gopm/config/gopm-config.json
pub static MACOS_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::home_dir().unwrap_or_else(|| {
        println!("❌ Failed to get the home directory");
        std::process::exit(1);
    });
    path.push("gopm/config/gopm-config.json");
    path
});

/// Path to the Linux config
/// File: gopm-config.json
/// PATH: ~/gopm/config/gopm-config.json
pub static LINUX_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = dirs::home_dir().unwrap_or_else(|| {
        println!("❌ Failed to get the home directory");
        std::process::exit(1);
    });
    path.push("gopm/config/gopm-config.json");
    path
});