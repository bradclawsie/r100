#[derive(Debug, PartialEq)]
struct Numbers {
    values: Vec<i64>,
}

impl Numbers {
    fn new(values: Vec<i64>) -> Self {
        Self {
            values,
        }
    }

    fn sum(&self) -> i64 {
        self.values.iter().sum()
    }
}

impl std::ops::Deref for Numbers {
    type Target = [i64];

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl AsRef<[i64]> for Numbers {
    fn as_ref(&self) -> &[i64] {
        &self.values
    }
}

fn average(values: &[i64]) -> Option<f64> {
    match values.len() {
        0 => None,
        n => Some(values.iter().sum::<i64>() as f64 / n as f64),
    }
}

fn count_positive<T: AsRef<[i64]>>(values: T) -> usize {
    values.as_ref().iter().filter(|&&x| x > 0).count()
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_new() {
        assert!(Numbers::new(vec![]).values.is_empty());
        assert!(Numbers::new(vec![]).values.first().is_none());
        assert_eq!(Numbers::new(vec![0, 1]).values.len(), 2);
        assert_eq!(Numbers::new(vec![0, 1]).values[1], 1);
        assert!(Numbers::new(vec![0, 1]).values.first().is_some_and(|x| *x == 0));
    }

    #[test]
    fn test_sum() {
        assert_eq!(Numbers::new(vec![]).sum(), 0);
        assert_eq!(Numbers::new(vec![1, 1]).sum(), 2);
    }

    #[test]
    fn test_deref() {
        let n0 = Numbers::new(vec![]);
        assert_eq!(*n0, []);
        let n1 = Numbers::new(vec![0,1,2]);
        assert_eq!(*n1, [0,1,2]);
        let n3 = Numbers::new(vec![10,20,30]);
        assert_eq!(n3.len(), 3);
        assert_eq!(n3[0], 10);
        assert_eq!(n3.first(), Some(&10));
        assert_eq!(n3.sum(), 60);
    }

    #[test]
    fn test_average() {
        let numbers = Numbers::new(vec![10, 20, 30]);
        assert_eq!(average(&numbers), Some(20.0));
    }

    #[test]
    fn test_count_positive() {
        let numbers = Numbers::new(vec![-3, 0, 4, 8]);
        assert_eq!(count_positive(&numbers), 2);
        assert_eq!(count_positive(&numbers), 2);
        assert_eq!(count_positive(vec![-3, 0, 4, 8]), 2);
        assert_eq!(count_positive(&[-3, 0, 4, 8]), 2);
    }
}
