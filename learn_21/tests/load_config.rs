use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process;

use learn_21::config::{Config, load_config, parse_config};
use learn_21::error::ConfigError;

#[test]
fn test_load_config() -> io::Result<()> {
    let mut file_path = env::temp_dir();
    file_path.push(format!("test_{}.txt", process::id()));

    {
        let mut file = File::create(&file_path)?;
        file.write_all(b"host=myhost\nport=1\nworkers=1\n")?;
    }

    assert!(load_config(&file_path).is_ok());
    assert!(load_config(std::path::Path::new("/bad/path")).is_err());
    
    fs::remove_file(&file_path)?;
    Ok(())
}
