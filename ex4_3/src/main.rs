use std::cmp::PartialEq;

struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title &&
            self.description == other.description &&
            self.status == other.status
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
        let t = Ticket{
            title: String::from("t"),
            description: String::from("d"),
            status: String::from("s"),
        };
        assert_eq!(t == t, true);
    }
}
