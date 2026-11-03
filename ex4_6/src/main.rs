pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn description(&self) -> &str {
        self.description.as_str()
    }

    fn status(&self) -> &str {
        self.status.as_str()
    }
}

fn main() {
    println!("");
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::any::{Any, TypeId};
    
    #[test]
    fn t0() {
        let t = Ticket{
            title: String::from("t"),
            description: String::from("d"),
            status: String::from("s"),
        };
        assert_eq!(TypeId::of::<str>(), t.title().type_id());
        assert_eq!(TypeId::of::<str>(), t.description().type_id());
        assert_eq!(TypeId::of::<str>(), t.status().type_id());
    }
}
