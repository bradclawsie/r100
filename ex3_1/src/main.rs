#[derive(Debug, Eq, PartialEq)]
pub struct Order {
    price: u32,
    quantity: u32,
}

impl Order {
    pub fn is_available(self: Self) -> bool {
        self.quantity != 0
    }
}

fn main() {
    let o = Order{price: 1, quantity: 1};
    println!("{}", o.is_available());
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn t0() {
        let o1 = Order{price: 1, quantity: 1};
        let o2: Order = Order{price: 1, quantity: 1};
        assert_eq!(o1,o2);
    }
}

