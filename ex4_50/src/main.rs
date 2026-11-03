use num::traits::PrimInt;

trait IsEven {
    fn is_even(&self) -> bool;
}

impl<T: PrimInt> IsEven for T {
    fn is_even(&self) -> bool {
        (*self & T::one()) == T::zero()
    }
}

fn main() {
    let four: u32 = 4;
    println!("{}", four.is_even());
}

#[cfg(test)]
mod tests {
    use super::IsEven;

    #[test]
    fn checks_signed_and_unsigned_integers() {
        assert!(4_u32.is_even());
        assert!(!3_u32.is_even());
        assert!(0_i32.is_even());
        assert!((-4_i32).is_even());
        assert!(!(-3_i32).is_even());
    }
}
