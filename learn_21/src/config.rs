use std::fs::read_to_string;
use std::num::NonZeroUsize;
use std::num::NonZeroU16;
use crate::error::ConfigError;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

pub fn parse_config(input: &str) -> Result<Config, ConfigError> {
    let mut host: Option<String> = None;
    let mut port: Option<NonZeroU16> = None;
    let mut workers: Option<NonZeroUsize> = None;
    let mut i: usize = 0;
    let valid_keys = ["host", "port", "workers"];
    
    for line in input.lines() {
        i += 1;
        let trimmed = line.trim();

        // skip
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;            
        }
        
        let (k, v) = trimmed.split_once('=').ok_or_else(|| ConfigError::InvalidLine(i))?;
        let key = k.trim();
        let val = v.trim();

        // one of the components is empty
        if key.is_empty() {
            return Err(ConfigError::InvalidLine(i)); 
        }
        if val.is_empty() {
            return Err(ConfigError::InvalidValue("".to_string()))
        }
        
        // bad key
        if !valid_keys.contains(&key) {
            return Err(ConfigError::UnknownField(key.to_string()))
        }
        
        if key == "host" {
            // already set
            if host.is_some() {
                return Err(ConfigError::DuplicateField("host".to_string()));
            }
            
            host = Some(val.to_string());
        }

        if key == "port" {
            // arlready set
            if port.is_some() {
                return Err(ConfigError::DuplicateField("port".to_string()));
            }

            if !val.chars().all(|c| c.is_ascii_digit()) {
                return Err(ConfigError::InvalidValue(val.to_string()));
            }
            
            port = Some(val.parse().
                map_err(|_| ConfigError::InvalidValue(val.to_string()))?);
        }

        if key == "workers" {
            // already set
            if workers.is_some() {
                return Err(ConfigError::DuplicateField("workers".to_string()));
            }

            if !val.chars().all(|c| c.is_ascii_digit()) {
                return Err(ConfigError::InvalidValue(val.to_string()));
            }

            workers = Some(val.parse().
                map_err(|_| ConfigError::InvalidValue(val.to_string()))?);
        }
    }

    if host.is_none() {
        return Err(ConfigError::MissingField("host".to_string()));
    }
    
    if port.is_none() {
        return Err(ConfigError::MissingField("port".to_string()));
    }

    if workers.is_none() {
        return Err(ConfigError::MissingField("workers".to_string()));
    }

    Ok(Config {
        host: host.ok_or_else(|| ConfigError::MissingField("host".into()))?,
        port: port.ok_or_else(|| ConfigError::MissingField("port".into()))?.get(),
        workers: workers.ok_or_else(|| ConfigError::MissingField("workers".into()))?.get(),
    })
}

pub fn load_config(path: &std::path::Path) -> Result<Config, ConfigError> {
    let contents: String = read_to_string(path).map_err(ConfigError::Io)?;
    parse_config(&contents)
}
 
