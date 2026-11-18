#[derive(Debug, PartialEq)]
struct Request {
    method: Method,
    path: String,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq)]
enum Method {
    Get,
    Post,
    Put,
    Delete,
}

struct RequestBuilder {
    request: Request,
}

#[derive(Debug, PartialEq)]
enum BuildError {
    MalformedPath,
    MalformedHeaderName(String),
    InvalidBody,
}

impl RequestBuilder {
    fn new(method: Method, path: String) -> Self {
        Self {
            request: Request {
                method,
                path,
                headers: Vec::new(),
                body: None,
            }
        }
    }

    fn header(self, name: String, value: String) -> Self {
        let mut builder = self;
        builder.request.headers.push((name,value));
        builder
    }

    fn body(self, data: Vec<u8>) -> Self {
        let mut builder = self;
        builder.request.body = Some(data);
        builder
    }

    fn build(self) -> Result<Request, BuildError> {
        // path must start with "/" and be non-empty
        if !(self.request.path.starts_with("/") &&
            self.request.path.len() > 1) {
                return Err(BuildError::MalformedPath);
            }

        // each header name char must be ascii alphanumeric or '-'
        for (name, _) in &self.request.headers {
            if !(!name.is_empty() &&
                name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')) {
                    return Err(BuildError::MalformedHeaderName(name.clone()));
            } 
        }

        // no body allowed on GET OR DELETE
        if self.request.body.is_some() && matches!(self.request.method, Method::Get | Method::Delete) {
            return Err(BuildError::InvalidBody);
        }
        Ok(self.request)
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
        let builder = RequestBuilder::new(Method::Get, "/".to_string());
        assert_eq!(builder.request.method, Method::Get);
        assert_eq!(builder.request.path, "/".to_string());
        assert!(builder.request.headers.is_empty());
        assert!(builder.request.body.is_none());
    }

    #[test]
    fn test_header() {
        let builder = RequestBuilder::new(Method::Get, "/".to_string());
        assert!(builder.request.headers.is_empty());
        let builder1 = builder.header("x".to_string(), "y".to_string());
        assert_eq!(builder1.request.headers.len(), 1);
        assert_eq!(builder1.request.headers[0], ("x".to_string(), "y".to_string()));
        let builder2 = builder1.header("x".to_string(), "y".to_string());
        assert_eq!(builder2.request.headers.len(), 2);
        assert_eq!(builder2.request.headers[0], ("x".to_string(), "y".to_string()));
        assert_eq!(builder2.request.headers[1], ("x".to_string(), "y".to_string()));
    }

    #[test]
    fn test_body() {
        let builder = RequestBuilder::new(Method::Get, "/".to_string());
        assert!(builder.request.body.is_none());
        let builder1 = builder.body(vec![1 as u8, 2 as u8]);
        assert_eq!(builder1.request.body, Some(vec![1 as u8, 2 as u8]));
        let builder2 = builder1.body(vec![1 as u8, 3 as u8]);
        assert_eq!(builder2.request.body, Some(vec![1 as u8, 3 as u8]));
    }

    #[test]
    fn test_build_malformed_path() {
        // path must start with /
        let builder0 = RequestBuilder::new(Method::Get, ".".to_string());
        assert_eq!(builder0.build(), Err(BuildError::MalformedPath));

        // path cannot be empty following /
        let builder1 = RequestBuilder::new(Method::Get, "/".to_string());
        assert_eq!(builder1.build(), Err(BuildError::MalformedPath));
    }

    #[test]
    fn test_build_malformed_header_name() {
        let builder0 = RequestBuilder::new(Method::Get, "/abc".to_string());
        assert!(builder0.request.headers.is_empty());
        let builder1 = builder0.header(" ".to_string(), "y".to_string());
        assert_eq!(builder1.build(), Err(BuildError::MalformedHeaderName(" ".to_string())));

        let builder2 = RequestBuilder::new(Method::Get, "/abc".to_string());
        assert!(builder2.request.headers.is_empty());
        let builder3 = builder2.header("".to_string(), "y".to_string());
        assert_eq!(builder3.build(), Err(BuildError::MalformedHeaderName("".to_string())));

        let builder4 = RequestBuilder::new(Method::Get, "/abc".to_string());
        assert!(builder4.request.headers.is_empty());
        let builder5 = builder4.header("|||".to_string(), "y".to_string());
        assert_eq!(builder5.build(), Err(BuildError::MalformedHeaderName("|||".to_string())));        
    }

    #[test]
    fn test_invalid_body() {
        let builder0 = RequestBuilder::new(Method::Get, "/abc".to_string());
        let builder1 = builder0.body(vec![0 as u8, 1 as u8]);
        assert_eq!(builder1.build(), Err(BuildError::InvalidBody));

        let builder2 = RequestBuilder::new(Method::Delete, "/abc".to_string());
        let builder3 = builder2.body(vec![0 as u8, 1 as u8]);
        assert_eq!(builder3.build(), Err(BuildError::InvalidBody));
    }

    #[test]
    fn test_build() {
        let builder0 = RequestBuilder::new(Method::Get, "/abc".to_string());
        assert!(builder0.build().is_ok());

        let builder1 = RequestBuilder::new(Method::Get, "/abc".to_string());
        let builder2 = builder1.header("a".to_string(), "b".to_string());
        let builder3 = builder2.header("a".to_string(), "c".to_string());
        assert_eq!(builder3.request.headers,
            vec![("a".to_string(), "b".to_string()),
                ("a".to_string(), "c".to_string())]);
        assert!(builder3.build().is_ok());

        let builder4 = RequestBuilder::new(Method::Post, "/abc".to_string());
        assert!(builder4.build().is_ok());

        let builder5 = RequestBuilder::new(Method::Put, "/abc".to_string());
        assert!(builder5.build().is_ok());

        let builder6 = RequestBuilder::new(Method::Post, "/abc".to_string());
        let builder7 = builder6.body(vec![0 as u8, 1 as u8]);
        assert!(builder7.build().is_ok());

        let builder8 = RequestBuilder::new(Method::Post, "/abc".to_string());
        let builder9 = builder8.body(vec![0 as u8, 1 as u8]);
        assert!(builder9.build().is_ok());
    }
}
