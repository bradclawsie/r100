trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        *self % 2 == 0
    }
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        *self % 2 == 0
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn t0() {
        let x: u32 = 4;
        assert_eq!(x.is_even(), true);

        let y: u32 = 5;
        assert_eq!(y.is_even(), false);
    }

    #[test]
    fn t1() {
        let x: i32 = 4;
        assert_eq!(x.is_even(), true);

        let y: i32 = 5;
        assert_eq!(y.is_even(), false);
    }
}
