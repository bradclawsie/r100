pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::mem::size_of;
    
    #[test]
    fn t0() {
        let mut t = Ticket{
            title:String::from("title"),
            description:String::from("desc"),
            status:String::from("status"),
        };

        println!("{:?}", size_of::<String>());
        println!("{:?}", size_of::<Ticket>());
        assert_eq!(true, true);
    }
}
