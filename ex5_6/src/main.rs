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
    fn new(title: &str, description: &str, status: Status) -> Result<Ticket, String> {
        if title.is_empty() {
            return Err(String::from("title is empty"));
        }
        Ok(
            Ticket {
                title: String::from(title),
                description: String::from(description),
                status,
            }
        )
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
        assert_eq!(Ticket::new(
            "",
            "d",
            Status::Done,
        ).is_err(), true);
    }
}
