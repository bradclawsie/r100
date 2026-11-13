struct History<T> {
    entries: Vec<T>,
    capacity: usize,
}

impl<T> History<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::<T>::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.entries.len() == self.capacity {
            self.entries.remove(0);
        }
        self.entries.push(value);
    }

    pub fn latest(&self) -> Option<&T> {
        self.entries.last()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.entries.pop()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    pub fn clear(&mut self) {
        self.entries.clear()
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
        let history = History::<u32>::new(4);
        assert_eq!(history.entries.len(), 0);
        assert_eq!(history.entries.capacity(), 4);
    }

    #[test]
    fn test_push() {
        let mut history = History::<u32>::new(2);
        assert_eq!(history.entries.len(), 0);
        history.push(11);
        assert_eq!(history.entries.len(), 1);
        assert_eq!(history.entries.as_slice(), &[11]);
        history.push(22);
        assert_eq!(history.entries.len(), 2);
        assert_eq!(history.entries.as_slice(), &[11, 22]);
        history.push(33);
        assert_eq!(history.entries.len(), 2);
        assert_eq!(history.entries.as_slice(), &[22, 33]);
    }

    #[test]
    fn test_latest() {
        let mut history = History::<u32>::new(2);
        assert_eq!(history.entries.len(), 0);
        history.push(11);
        assert_eq!(history.entries.len(), 1);
        assert!(history.latest().is_some_and(|x| *x == 11));
        history.push(22);
        assert_eq!(history.entries.len(), 2);
        assert!(history.latest().is_some_and(|x| *x == 22));
        history.push(33);
        assert_eq!(history.entries.len(), 2);
        assert!(history.latest().is_some_and(|x| *x == 33));
    }

    #[test]
    fn test_pop() {
        let mut history = History::<u32>::new(2);
        assert_eq!(history.entries.len(), 0);
        history.push(11);
        history.push(22);
        assert!(history.pop().is_some_and(|x| x == 22));
        assert!(history.pop().is_some_and(|x| x == 11));
        assert!(history.pop().is_none());
    }

    #[test]
    fn test_len() {
        let mut history = History::<u32>::new(2);
        assert_eq!(history.len(), 0);
        history.push(11);
        assert_eq!(history.len(), 1);
        history.push(22);
        assert_eq!(history.len(), 2);
    }
    
    #[test]
    fn test_clear() {
        let mut history = History::<u32>::new(2);
        assert_eq!(history.entries.len(), 0);
        history.push(11);
        history.push(22);
        history.clear();
        assert_eq!(history.len(), 0);
    }
}
