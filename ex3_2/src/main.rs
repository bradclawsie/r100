#[derive(Debug)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: &str, description: &str, status: &str) -> Self {
        if !(status == "To-Do" || status == "In Progress" || status == "Done") {
            panic!("status not correct");
        }
        
        if title.is_empty() || description.is_empty() {
            panic!("title or description is empty");
        }

        if title.len() > 50 {
            panic!("title too long");
        }

        if description.len() > 500 {
            panic!("description too long");
        }

        Self{
            title: String::from(title),
            description: String::from(description),
            status: String::from(status),
        }
    }
}

fn main() {
    let ticket = Ticket::new("title","desc","Done");
    println!("{:?}{:?}{:?}", ticket.title, ticket.description, ticket.status);
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    #[should_panic(expected="status not correct")]
    fn t0() {
        Ticket::new("title","desc","status");
    }
}

