#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    MissingField(String),
    InvalidLine(usize),
    DuplicateField(String),
    InvalidValue(String),
    UnknownField(String),
}
