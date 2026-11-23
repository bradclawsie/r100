#[derive(Debug, PartialEq, Eq)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseVersionError {
    WrongCount,
    EmptyComponent,
    InvalidComponent,
    OutOfRange,
}

impl std::fmt::Display for ParseVersionError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let msg = match self {
            ParseVersionError::WrongCount => "wrong component count",
            ParseVersionError::EmptyComponent => "empty component",
            ParseVersionError::InvalidComponent => "invalid component",
            ParseVersionError::OutOfRange => "out of range",
        };
        write!(f, "{}", msg)
    }
}

impl std::str::FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elts: Vec<&str> = s.split('.').collect();
        if elts.len() != 3 {
            return Err(ParseVersionError::WrongCount);
        }
        
        for i in 0..3 {
            if elts[i].is_empty() {
                return Err(ParseVersionError::EmptyComponent);
            }
        }

        for i in 0..3 {
            if !elts[i].bytes().all(|b| b.is_ascii_digit()) {
                return Err(ParseVersionError::InvalidComponent);
            }
        }
        
        let major = elts[0].parse::<u32>().map_err(|_|
            ParseVersionError::OutOfRange)?;
        let minor = elts[1].parse::<u32>().map_err(|_|
            ParseVersionError::OutOfRange)?;
        let patch = elts[2].parse::<u32>().map_err(|_|
            ParseVersionError::OutOfRange)?;
        Ok(Version::new(major, minor, patch))
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::*;

    #[test]
    fn test_new() {
        let v = Version::new(1,2,3);
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_display() {
        let v = Version::new(1,2,3);
        assert_eq!(v.to_string(), "1.2.3");
        assert_eq!(format!("{}", v), "1.2.3");
    }

    #[test]
    fn test_from_str() {
        let v = Version::new(1,2,3);
        assert!(Version::from_str("1.2.3").is_ok_and(|x| x == v));
        assert!(Version::from_str("0.0.0").is_ok_and(|x| x == Version::new(0,0,0)));
        assert!(Version::from_str("01.02.03").is_ok_and(|x| x == v));
        assert!(Version::from_str("4294967295.0.0").is_ok_and(|x| x.major == u32::MAX));
        assert!(Version::from_str("1.2").is_err_and(|x| x == ParseVersionError::WrongCount));
        assert!(Version::from_str("1.2.3.4").is_err_and(|x| x == ParseVersionError::WrongCount));
        assert!(Version::from_str("1..3").is_err_and(|x| x == ParseVersionError::EmptyComponent));
        assert!(Version::from_str("1.a.3").is_err_and(|x| x == ParseVersionError::InvalidComponent));
        assert!(Version::from_str("1.-2.3").is_err_and(|x| x == ParseVersionError::InvalidComponent));
        assert!(Version::from_str("1.4294967296.3").is_err_and(|x| x == ParseVersionError::OutOfRange));
        assert!(Version::from_str("1.99999999999999999999999.3").is_err_and(|x| x == ParseVersionError::OutOfRange));
        assert!(Version::from_str(" 1.2.3").is_err_and(|x| x == ParseVersionError::InvalidComponent));
    }

    #[test]
    fn test_round_trip() {
        let version: Version = "1.24.3".parse().unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 24);
        assert_eq!(version.patch, 3);
        assert_eq!(version.to_string(), "1.24.3");

        let version1 = Version::from_str("2.5.10").unwrap();
        assert_eq!(version1, Version::new(2, 5, 10));

        let original = Version::new(10, 20, 30);
        
        let text = original.to_string();
        let parsed: Version = text.parse().unwrap();
        
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_precedence() {
        assert_eq!(
            "999999999999999..3".parse::<Version>(),
            Err(ParseVersionError::EmptyComponent)
        );
        
        assert_eq!(
            "999999999999999.abc.3".parse::<Version>(),
            Err(ParseVersionError::InvalidComponent)
        );
    }
}
