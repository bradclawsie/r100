pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(v: u32) -> Self {
        Self{value:v}
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
        let n: u32 = 5;
        let w: WrappingU32 = n.into();
        assert_eq!(w.value,n);
    }
}
