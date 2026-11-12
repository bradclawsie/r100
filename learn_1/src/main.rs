struct Inventory {
    items: Vec<String>,
}

impl Inventory {
    // `new` creates a new Inventory instance.
    pub fn new() -> Self {
        Self{ items: Vec::new() }
    }

    // `add` inserts a new item.
    pub fn add(&mut self, item: String) {
        self.items.push(item)
    }

    // `contains` returns true if `item` is in `self.items`.
    pub fn contains(&self, item: &str) -> bool {
        self.items.contains(&String::from(item))
    }

    // `remove` removes `item` from `self.items` if present.
    pub fn remove(&mut self, item: &str) -> Option<String> {
        let index: usize = self.items.iter().position(|x| x.as_str() == item)?;
        Some(self.items.remove(index))
    }

    // `list` returns a slice of the inventory items.
    pub fn list(&self) -> &[String] {
        self.items.as_slice()
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
        let inventory = Inventory::new();
        let _ = inventory.items;
        assert_eq!(true, true);
    }

    #[test]
    fn test_add() {
        let mut inventory = Inventory::new();
        inventory.add(String::from("a"));
        assert_eq!(inventory.contains(&String::from("a")), true);
    }
    
    #[test]
    fn test_contains() {
        let mut inventory = Inventory::new();
        inventory.add(String::from("a"));
        assert_eq!(inventory.contains(&String::from("a")), true);
        assert_eq!(inventory.contains(&String::from("b")), false);
    }

    #[test]
    fn test_remove() {
        let mut inventory = Inventory::new();
        inventory.add(String::from("a"));
        let removed = inventory.remove(&String::from("a"));
        assert!(removed.is_some_and(|x| x == String::from("a")));
        let miss = inventory.remove(&String::from("b"));
        assert!(miss.is_none());

        inventory.add(String::from("a"));
        inventory.add(String::from("b"));
        inventory.add(String::from("a"));
        assert_eq!(inventory.list(), &["a","b","a"]);
        let removed2 = inventory.remove(&String::from("a"));
        assert!(removed2.is_some_and(|x| x == String::from("a")));
        assert_eq!(inventory.list(), &["b","a"]);
    }

    #[test]
    fn test_list() {
        let mut inventory = Inventory::new();
        inventory.add(String::from("a"));
        assert_eq!(inventory.list(), &[String::from("a")]);
    }
}
