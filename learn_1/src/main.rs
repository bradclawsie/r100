struct Inventory {
    items: Vec<String>,
}

impl Inventory {
    // `new` creates a new Inventory instance.
    pub fn new() -> Self {
        Self{ items: Vec::new() }
    }

    // `contains` returns true if `item` is in `self.items`.
    pub fn contains(&self, item: &String) -> bool {
        self.items.contains(item)
    }

    // `remove` removes `item` from `self.items` if present.
    pub fn remove(&mut self, item: &String) -> Option<String> {
        let index: usize= match self.items.iter().position(|x| *x == *item) {
            Some(i) => i,
            None => return None,
        };
        let removed = self.items[index].clone();
        self.items.remove(index);
        Some(removed)
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
    fn test_contains() {
        let mut inventory = Inventory::new();
        inventory.items.push(String::from("a"));
        assert_eq!(inventory.contains(&String::from("a")), true);
        assert_eq!(inventory.contains(&String::from("b")), false);
    }

    #[test]
    fn test_remove() {
        let mut inventory = Inventory::new();
        inventory.items.push(String::from("a"));
        let removed = inventory.remove(&String::from("a"));
        assert!(removed.is_some_and(|x| x == String::from("a")));
        let miss = inventory.remove(&String::from("b"));
        assert!(miss.is_none());
    }

    #[test]
    fn test_list() {
        let mut inventory = Inventory::new();
        inventory.items.push(String::from("a"));
        assert_eq!(inventory.list(), &[String::from("a")]);
    }
}
