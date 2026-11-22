use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Username(String);

impl Username {
    fn new(name: String) -> Self {
        Self(name)
    }
}

impl Borrow<str> for Username {
    fn borrow(&self) -> &str {
        &self.0
    }
}

struct UserRegistry {
    users: HashMap<Username, u64>,
}

impl UserRegistry {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    fn insert(&mut self, username: Username, id: u64) -> Option<u64> {
        self.users.insert(username, id)
    }

    fn get(&self, username: &str) -> Option<u64> {
        self.users.get(username).copied()
    }

    fn remove(&mut self, username: &str) -> Option<u64> {
        self.users.remove(username)
    }

    fn len(&self) -> usize {
        self.users.len()
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_username_new() {
        let un = Username::new("Joe".to_string());
        assert_eq!(un.0, "Joe".to_string());
    }

    #[test]
    fn test_userregistry_new() {
        let ur = UserRegistry::new();
        assert!(ur.users.is_empty());
    }

    #[test]
    fn test_userregistry_ops() {
        let mut ur = UserRegistry::new();
        assert!(ur.users.is_empty());

        assert!(&ur.insert(Username::new("Joe".to_string()),1).is_none());
        assert!(&ur.insert(Username::new("Joe".to_string()),2).is_some_and(|x| x == 1));
        assert!(&ur.get("Joe").is_some_and(|x| x == 2));
        assert!(&ur.get("Jim").is_none());
        assert!(&ur.insert(Username::new("Jim".to_string()),3).is_none());
        assert_eq!(ur.len(), 2 as usize);
        assert!(&ur.remove("Joe").is_some_and(|x| x == 2));
        assert_eq!(ur.len(), 1 as usize);
        assert!(&ur.remove("Jay").is_none());
        assert_eq!(ur.len(), 1 as usize);
        assert!(&ur.insert(Username::new("Joe".to_string()),2).is_none());
        assert_eq!(ur.len(), 2 as usize);
        assert!(&ur.get("Joe".to_string().as_str()).is_some_and(|x| x == 2));
        
    }
}
