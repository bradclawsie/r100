struct Document {
    title: String,
    body: String,
}

struct DocumentView<'a> {
    title: &'a str,
    body: &'a str,
}

impl Document {
    pub fn new(title: String, body: String) -> Self {
        Self {
            title,
            body,
        }
    }

    pub fn view(&self) -> DocumentView<'_> {
        DocumentView{
            title: &self.title,
            body: &self.body,
        }
    }
}

impl<'a> DocumentView<'a> {
    pub fn title(&self) -> &str {
        self.title
    }

    pub fn body(&self) -> &str {
        self.body
    }

    pub fn word_count(&self) -> usize {
        self.body.split_whitespace().count()
    }

    pub fn summary(&self, n: usize) -> &str {
        match self.body.char_indices().nth(n) {
	    Some((idx, _)) => &self.body[..idx],
            None => &self.body,
	}
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_document_new() {
        let document = Document::new("title".to_string(), "body".to_string());
        assert_eq!(document.title, "title".to_string());
        assert_eq!(document.body, "body".to_string());
    }

    #[test]
    fn test_document_view() {
        let document = Document::new("title".to_string(), "body".to_string());
        let view = document.view();
        assert_eq!(view.title, "title");
        assert_eq!(view.body, "body");
    }

    #[test]
    fn test_document_view_title() {
        let document = Document::new("title".to_string(), "body".to_string());
        let view = document.view();
        assert_eq!(view.title(), "title");
    }

    #[test]
    fn test_document_view_body() {
        let document = Document::new("title".to_string(), "body".to_string());
        let view = document.view();
        assert_eq!(view.body(), "body");
    }

    #[test]
    fn test_document_view_word_count() {
        let empty = Document::new("title".to_string(), "".to_string());
        let empty_view = empty.view();
        assert_eq!(empty_view.word_count(), 0);
        let document = Document::new("title".to_string(), "body body body".to_string());
        let view = document.view();
        assert_eq!(view.word_count(), 3);
    }

    #[test]
    fn test_document_view_summary() {
        let document0 = Document::new("title".to_string(), "hello world".to_string());
        let view0 = document0.view();
        assert_eq!(view0.summary(5), "hello");
        let document1 = Document::new("title".to_string(), "café ☕".to_string());
        let view1 = document1.view();
        assert_eq!(view1.summary(4), "café");
        let document2 = Document::new("title".to_string(), "café ☕".to_string());
        let view2 = document2.view();
        assert_eq!(view1.summary(99), "café ☕");
    }

}
