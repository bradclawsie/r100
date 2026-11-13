struct Slot<T> {
    value: Option<T>,
}

impl<T> Slot<T> {
    pub fn new() -> Self {
        Self {
            value: None,
        }
    }

    pub fn insert(&mut self, value: T) -> Option<T> {
        let current_value = self.value.take();
        self.value = Some(value);
        current_value
    }

    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn take(&mut self) -> Option<T> {
        self.value.take()
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_none()
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
        assert!(Slot::<u32>::new().value.is_none());
    }

    struct Record {
        id: u32,
    }
    
    #[test]
    fn test_insert() {
        let mut slot = Slot::<Record>::new();
        let prev0 = slot.insert(Record{id:1});
        assert!(prev0.is_none());
        let prev1 = slot.insert(Record{id:2});
        assert!(prev1.is_some_and(|x| x.id == 1))
    }

    #[test]
    fn test_get() {
        let mut slot = Slot::<Record>::new();
        assert!(slot.get().is_none());
        let prev0 = slot.insert(Record{id:1});
        assert!(slot.get().is_some_and(|x| x.id == 1))
    }

    #[test]
    fn test_take() {
        let mut slot = Slot::<Record>::new();
        assert!(slot.take().is_none());
        let _ = slot.insert(Record{id:1});
        assert!(slot.take().is_some_and(|x| x.id == 1));
        assert!(slot.get().is_none());
    }

    #[test]
    fn test_is_empty() {
        let mut slot = Slot::<Record>::new();
        assert!(slot.is_empty());
        let _ = slot.insert(Record{id:1});
        assert!(!slot.is_empty());
    }
}
