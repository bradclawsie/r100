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
    fn assigned_to(&self) -> String {
        match self.status {
            Status::InProgress { ref assigned_to } => {
                assigned_to.clone()
            }
            _ => panic!("not in progress")
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
            status: Status::InProgress{
                assigned_to: String::from("at")
            },
        };
        assert_eq!(t.assigned_to(), "at");
    }

    #[test]
    #[should_panic(expected = "not in progress")]
    fn t1 () {
        let t = Ticket{
            title: String::from("t"),
            description: String::from("d"),
            status: Status::Done,
        };
        _ = t.assigned_to();
    }
}
