use learn_21::config::{Config, parse_config};
use learn_21::error::ConfigError;

#[test]
fn test_ok() {
    let ok0 = "\n# comment\n host=my-host\nport=1  \n  workers=1 \n";
    assert!(parse_config(ok0).is_ok_and(|x| x ==
        Config{host:"my-host".to_string(), port:1, workers:1}));

    let ok1 = format!("host=myhost-012-\nport=65535  \n  workers={} \n", usize::MAX);
    assert!(parse_config(&ok1).is_ok_and(|x| x ==
        Config{host:"myhost-012-".to_string(), port:65535, workers:usize::MAX}));

    let ok2 = "port=1\n# comment\n host=my-host  \n\n  workers=1 \n";
    assert!(parse_config(ok2).is_ok_and(|x| x ==
        Config{host:"my-host".to_string(), port:1, workers:1}));

    let ok3 = "port=1\n# comment\n host=my=host\n\n  workers=1 \n";
    assert!(parse_config(ok3).is_ok_and(|x| x ==
        Config{host:"my=host".to_string(), port:1, workers:1}));

}

#[test]
fn test_invalid_line() {
    let invalid0 = "host=hello\nport\nworkers=1\n";
    dbg!(parse_config(invalid0));
    assert!(matches!(
        parse_config(invalid0),
        Err(ConfigError::InvalidLine(2))
    ));
}

#[test]
fn test_unknown_field() {
    let invalid0 = "host=hello\n__port=1\nworkers=1\n";
    assert!(matches!(
        parse_config(invalid0),
	Err(ConfigError::UnknownField(s)) if s == "__port"
    ));

    let invalid1 = "host=hello\nport__=1\n";
    assert!(matches!(
        parse_config(invalid1),
	Err(ConfigError::UnknownField(s)) if s == "port__"
    ));
 }

#[test]
fn test_duplicate_field() {
    let invalid0 = "host=hello\nport=1\nworkers=1\nhost=hello\n";
    assert!(matches!(
        parse_config(invalid0),
	Err(ConfigError::DuplicateField(s)) if s == "host"
    ));

    let invalid1 = "host=hello\nport=1\nworkers=1\nport=2\n";
    assert!(matches!(
        parse_config(invalid1),
	Err(ConfigError::DuplicateField(s)) if s == "port"
    ));

    let invalid2 = "host=hello\nport=1\nworkers=1\nworkers=4\n";
    assert!(matches!(
        parse_config(invalid2),
	Err(ConfigError::DuplicateField(s)) if s == "workers"
    ));
}

#[test]
fn test_invalid_value() {
    let invalid0 = "host=hello...&&&\nport=1\nworkers=1\n";
    assert!(matches!(
        parse_config(invalid0),
	Err(ConfigError::InvalidValue(s)) if s == "hello...&&&"
    ));

    let invalid1 = "host=hello\nport=0\nworkers=1\n";
    assert!(matches!(
        parse_config(invalid1),
	Err(ConfigError::InvalidValue(s)) if s == "0"
    ));

    let invalid2 = "host=hello\nport=1\nworkers=0\n";
    assert!(matches!(
        parse_config(invalid2),
	Err(ConfigError::InvalidValue(s)) if s == "0"
    ));

    let invalid3 = "host=hello\nport=99999999999999999999\nworkers=1\n";
    assert!(matches!(
        parse_config(invalid3),
	Err(ConfigError::InvalidValue(s)) if s == "99999999999999999999"
    ));

    let invalid4 = "host=hello\nport=1\nworkers=999999999999999999999999999999999\n";
    assert!(matches!(
        parse_config(invalid4),
	Err(ConfigError::InvalidValue(s)) if s == "999999999999999999999999999999999"
    ));

    let invalid5 = "host=hello\nport=abc\nworkers=1\n";
    assert!(matches!(
        parse_config(invalid5),
	Err(ConfigError::InvalidValue(s)) if s == "abc"
    ));

    let invalid6 = "host=hello\nport=1\nworkers=def\n";
    assert!(matches!(
        parse_config(invalid6),
	Err(ConfigError::InvalidValue(s)) if s == "def"
    ));
}

#[test]
fn test_missing_field() {
    let invalid0 = "port=1\nworkers=1\n";
    assert!(matches!(
        parse_config(invalid0),
	Err(ConfigError::MissingField(s)) if s == "host"
    ));

    let invalid1 = "host=hello\nworkers=1\n";
    assert!(matches!(
        parse_config(invalid1),
	Err(ConfigError::MissingField(s)) if s == "port"
    ));

    let invalid2 = "host=hello\nport=1\n";
    assert!(matches!(
        parse_config(invalid2),
	Err(ConfigError::MissingField(s)) if s == "workers"
    ));
}
