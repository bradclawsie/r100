#[derive(Debug, PartialEq)]
enum Outcome<T, E> {
    Pending,
    Success(T),
    Failure(E),
}

#[derive(Debug, PartialEq)]
struct ResultSlot<T, E> {
    outcome: Outcome<T, E>,
}

impl<T, E> ResultSlot<T, E> {
    fn new() -> Self {
        Self {
            outcome: Outcome::Pending,
        }
    }

    fn succeed(&mut self, value: T) -> bool {
        if !matches!(self.outcome, Outcome::Pending) {
            return false;
        }
        self.outcome = Outcome::Success(value);
        true
    }

    fn fail(&mut self, error: E) -> bool {
        if !matches!(self.outcome, Outcome::Pending) {
            return false;
        }
        self.outcome = Outcome::Failure(error);
        true
    }

    fn success(&self) -> Option<&T> {
        match &self.outcome {
            Outcome::Success(v) => Some(v),
            _ => None,
        }
    }

    fn failure(&self) -> Option<&E> {
        match &self.outcome {
            Outcome::Failure(v) => Some(v),
            _ => None,
        }
    }

    fn take_success(&mut self) -> Option<T> {
        if !matches!(self.outcome, Outcome::Success(..)) {
            return None;
        }
        match std::mem::replace(&mut self.outcome, Outcome::Pending) {
            Outcome::Success(val) => Some(val),
            _ => unreachable!(),
        } 
    }

    fn take_failure(&mut self) -> Option<E> {
        if !matches!(self.outcome, Outcome::Failure(..)) {
            return None;
        }
        match std::mem::replace(&mut self.outcome, Outcome::Pending) {
            Outcome::Failure(val) => Some(val),
            _ => unreachable!(),
        }
    }

    fn reset(&mut self) {
        self.outcome = Outcome::Pending
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
        assert_eq!(ResultSlot::<(),()>::new().outcome, Outcome::Pending);
    }

    #[test]
    fn test_succeed() {
        let mut rs = ResultSlot::<bool,()>::new();
        assert!(rs.succeed(true));
        assert_eq!(rs.outcome, Outcome::Success(true));
        assert!(!rs.succeed(true));
        rs = ResultSlot::<bool,()>::new();
        assert!(rs.fail(()));
        assert!(!rs.succeed(true));
    }

    #[test]
    fn test_fail() {
        let mut rs = ResultSlot::<bool,()>::new();
        assert!(rs.fail(()));
        assert_eq!(rs.outcome, Outcome::Failure(()));
        assert!(!rs.fail(()));
        rs = ResultSlot::<bool,()>::new();
        assert!(rs.succeed(true));
        assert!(!rs.fail(()));
    }

    #[test]
    fn test_success() {
        let mut rs = ResultSlot::<bool, bool>::new();
        assert!(rs.success().is_none());
        assert!(rs.succeed(true));
        assert!(rs.success().is_some_and(|x| *x == true));
        rs = ResultSlot::<bool, bool>::new();
        assert!(rs.fail(true));
        assert!(rs.success().is_none());
    }

    #[test]
    fn test_failure() {
        let mut rs = ResultSlot::<bool, bool>::new();
        assert!(rs.failure().is_none());
        assert!(rs.fail(true));
        assert!(rs.failure().is_some_and(|x| *x == true));
        rs = ResultSlot::<bool, bool>::new();
        assert!(rs.succeed(true));
        assert!(rs.failure().is_none());
    }

    #[test]
    fn test_take_success() {
        let mut rs = ResultSlot::<bool, bool>::new();
        assert!(rs.success().is_none());
        assert!(rs.succeed(true));
        assert!(rs.take_success().is_some_and(|x| x == true));
        rs = ResultSlot::<bool, bool>::new();
        assert!(rs.fail(true));
        assert!(rs.success().is_none());
    }

    #[test]
    fn test_take_failure() {
        let mut rs = ResultSlot::<bool, bool>::new();
        assert!(rs.failure().is_none());
        assert!(rs.fail(true));
        assert!(rs.take_failure().is_some_and(|x| x == true));
        rs = ResultSlot::<bool, bool>::new();
        assert!(rs.succeed(true));
        assert!(rs.failure().is_none());
    }

    #[test]
    fn test_reset() {
        let mut rs = ResultSlot::<bool, bool>::new();
        assert!(rs.fail(true));
        assert!(matches!(rs.outcome, Outcome::Failure(..)));
        rs.reset();
        assert!(matches!(rs.outcome, Outcome::Pending));
        rs = ResultSlot::<bool, bool>::new();
        assert!(rs.succeed(true));
        assert!(matches!(rs.outcome, Outcome::Success(..)));
        rs.reset();
        assert!(matches!(rs.outcome, Outcome::Pending));
    }
}
