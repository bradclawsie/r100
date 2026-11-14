struct PairStore<T> {
    values: Vec<T>,
}

impl<T> PairStore<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value)
    }

    pub fn get_pair_mut(&mut self, a: usize, b: usize) -> Option<(&mut T, &mut T)> {
        if a == b || a >= self.values.len() || b >= self.values.len() {
            return None
        }
        if a < b {
	    let (left, right) = self.values.split_at_mut(b);
  	    Some((&mut left[a], &mut right[0]))
	} else {
            let (left, right) = self.values.split_at_mut(a);
            Some((&mut right[0], &mut left[b]))
        }
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
        assert_eq!(PairStore::<String>::new().values.len(), 0);
    }

    #[test]
    fn test_push() {
        let mut pairstore = PairStore::<String>::new();
        assert_eq!(pairstore.values.len(), 0);
        pairstore.push("abc".to_string());
        assert_eq!(pairstore.values.len(), 1);
    }

    #[test]
    fn test_get_pair_mut() {
        let mut pairstore = PairStore::<String>::new();
        assert!(pairstore.get_pair_mut(0,0).is_none());
        pairstore.push("a".to_string());
        pairstore.push("b".to_string());
        pairstore.push("c".to_string());
        pairstore.push("d".to_string());
        assert!(pairstore.get_pair_mut(4,0).is_none());
        assert!(pairstore.get_pair_mut(0,4).is_none());

        let Some((a,b)) = pairstore.get_pair_mut(0,1) else {
            panic!("expected Some");
        };
        assert_eq!(*a, "a".to_string());
        assert_eq!(*b, "b".to_string());
        
        *a = "A".to_string();
        *b = "B".to_string();
        let Some((A,B)) = pairstore.get_pair_mut(0,1) else {
            panic!("expected Some");
        };
        assert_eq!(*A, "A".to_string());
        assert_eq!(*B, "B".to_string());

        
        let Some((d,c)) = pairstore.get_pair_mut(3,2) else {
            panic!("expected Some");
        };
        assert_eq!(*c, "c".to_string());
        assert_eq!(*d, "d".to_string());
    }
 }
