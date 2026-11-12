struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    author: String,
}

impl Library {
    fn new() -> Self {
        Self{
            books: Vec::new(),
        }
    }

    fn add(&mut self, book: Book) {
        // libraries are allowed to have copies of the same book
        self.books.push(book)
    }

    fn find(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|&x| x.title == title)
    }

    fn take(&mut self, title: &str) -> Option<Book> {
        let index = self.books.iter().position(|x| x.title == title)?;
        Some(self.books.remove(index))
    }

    fn transfer(&mut self, destination: &mut Library, title: &str) -> bool {
        if let Some(book) = self.take(title) {
            destination.add(book);
            return true;
        }
        false
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_new() {
        let library = Library::new();
        assert_eq!(library.books.len(), 0);
    }

    #[test]
    fn test_add() {
        let mut library = Library::new();
        assert_eq!(library.books.len(), 0);
        library.add(Book{
            title: String::from("some book"),
            author: String::from("some author"),
        });
        assert_eq!(library.books.len(), 1);
    }

    #[test]
    fn test_find() {
        let mut library = Library::new();
        library.add(Book{
            title: String::from("some book"),
            author: String::from("some author"),
        });
        assert!(library.find("some book").is_some());
        assert!(library.find("other book").is_none());
    }

    #[test]
    fn test_take() {
        let mut library = Library::new();
        library.add(Book{
            title: String::from("some book"),
            author: String::from("some author"),
        });
        assert!(library.take("some book").is_some());
        assert!(library.take("other book").is_none());
    }

    #[test]
    fn test_transfer() {
        let mut library = Library::new();
        library.add(Book{
            title: String::from("some book"),
            author: String::from("some author"),
        });
        let mut destination = Library::new();
        assert!(library.transfer(&mut destination, "some book"));
        assert!(destination.find("some book").is_some());
        assert!(library.find("some book").is_none());
    }
    
}
