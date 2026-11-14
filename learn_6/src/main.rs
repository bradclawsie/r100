struct Interner {
    strings: Vec<String>,
}

impl Interner {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
        }
    }

    pub fn lookup(&self, s: &str) -> Option<&String> {
        self.strings.iter().find(|x| *x == s)
    }

    pub fn intern(&mut self, s: &str) -> &String {
        if let Some(idx) = self.strings.iter().position(|x| x == s) {
	    return &self.strings[idx];
	}
	self.strings.push(String::from(s));
        self.strings.last().unwrap()
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_new() {
        assert_eq!(Interner::new().strings.len(), 0);
    }

    #[test]
    fn test_lookup() {
        let mut interner = Interner::new();
        interner.strings.push(String::from("abc"));
        assert!(interner.lookup("abc").is_some_and(|x| x == "abc"));
        assert!(interner.lookup("def").is_none());
    }

    #[test]
    fn test_intern() {
        let mut interner = Interner::new();
        assert_eq!(interner.strings.len(), 0);
        assert_eq!(interner.intern("a"), "a");
        assert_eq!(interner.strings.len(), 1);
        assert_eq!(interner.intern("b"), "b");
        assert_eq!(interner.strings.len(), 2);
        assert_eq!(interner.intern("b"), "b");
        assert_eq!(interner.strings.len(), 2);        
    }
}
