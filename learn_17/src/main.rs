#[derive(Debug, PartialEq)]
struct Document {
    title: String,
    body: String,
}

impl Document {
    fn new(title: String, body: String) -> Self {
        Self {
            title,
            body,
        }
    }

    fn title(&self) -> &str {
        self.title.as_ref()
    }

    fn body(&self) -> &str {
        self.body.as_ref()
    }
}

impl AsRef<str> for Document {
    fn as_ref(&self) -> &str {
        self.body()
    }
}

fn word_count<T: AsRef<str>>(value: T) -> usize {
    value.as_ref().split_whitespace().count()
}

fn contains_text<T, U>(haystack: T, needle: U) -> bool
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    haystack.as_ref().contains(needle.as_ref())
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_document() {
        let document = Document::new("title".to_string(), "body".to_string());
        assert_eq!(document.title(), "title");
        assert_eq!(document.body(), "body");
        assert_eq!(document.as_ref(), document.body());
    }

    #[test]
    fn test_word_count() {
        let document = Document::new("title".to_string(), "one two three".to_string());
        assert_eq!(word_count(&document), 3);
        assert_eq!(word_count(document), 3);
        assert_eq!(word_count("one two three".to_string()), 3);
        assert_eq!(word_count("one two three"), 3);
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("   "), 0);
        assert_eq!(word_count("one\ttwo\nthree"), 3);
    }

    #[test]
    fn test_contains_text() {
        assert!(contains_text("a b c", "a"));
        assert!(contains_text("a b c", ""));
        assert!(!contains_text("a b c", "z"));
        assert!(!contains_text("a b c", "A"));
        assert!(contains_text("a b c".to_string(), "a".to_string()));
        assert!(!contains_text("a b c".to_string(), "z".to_string()));
        let d0 = Document::new("t".to_string(), "a b c".to_string());
        let d1 = Document::new("t".to_string(), "a".to_string());
        assert!(contains_text(&d0, d1));
        assert!(!contains_text(&d0, "z".to_string()));        
    }

    #[test]
    fn test_document_reuse() {
        let doc = Document::new(
            "Notes".to_string(),
            "Rust ownership matters".to_string(),
        );
        
        assert_eq!(word_count(&doc), 3);
        assert!(contains_text(&doc, "ownership"));
        assert!(contains_text(&doc, "Rust"));
        
        assert_eq!(doc.title(), "Notes");
        assert_eq!(doc.body(), "Rust ownership matters");
    }
}
