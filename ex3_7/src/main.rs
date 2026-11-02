pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn status(&self) -> String {
        self.status.clone()
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title); 
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
    }

    pub fn set_status(&mut self, status: &str) {
        self.status = String::from(status);
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
        let mut t = Ticket{
            title:String::from("title"),
            description:String::from("desc"),
            status:String::from("status"),
        };
        t.set_title("boo");
        assert_eq!(t.title, "boo");
    }
}
