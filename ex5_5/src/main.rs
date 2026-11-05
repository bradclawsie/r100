#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn assigned_to(&self) -> Option<String> {
        match self.status {
            Status::InProgress { ref assigned_to } => Some(assigned_to.clone()),
            _ => None,
        }
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
            status: Status::InProgress { assigned_to: String::from("me")},
        };
        assert_eq!(t.assigned_to(), Some(String::from("me")));
    }
}
