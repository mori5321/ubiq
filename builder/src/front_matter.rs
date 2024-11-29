use serde;
use serde_yaml;

#[derive(Debug)]
pub enum FrontMatterError {
    MissingBeginningLine,
    MissingEndingLine,
    InvalidYaml(serde_yaml::Error),
}

impl PartialEq for FrontMatterError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FrontMatterError::MissingBeginningLine, FrontMatterError::MissingBeginningLine) => {
                true
            }
            (FrontMatterError::MissingEndingLine, FrontMatterError::MissingEndingLine) => true,
            (FrontMatterError::InvalidYaml(_), FrontMatterError::InvalidYaml(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Parsed<'a, T: serde::de::DeserializeOwned> {
    pub headers: T,
    pub body: &'a str,
}

pub fn parse<T>(text: &str) -> Result<Parsed<T>, FrontMatterError>
where
    T: serde::de::DeserializeOwned,
{
    let line_pattern_lf = "---\n";
    let line_pattern_crlf = "---\r\n";

    if let Some(slice) = text.strip_prefix(line_pattern_lf) {
        let index_of_ending_line = slice
            .find(line_pattern_lf)
            .ok_or(FrontMatterError::MissingEndingLine)?;
        let headers = serde_yaml::from_str(&slice[..index_of_ending_line])
            .map_err(FrontMatterError::InvalidYaml)?;
        let body = &slice[(index_of_ending_line + line_pattern_lf.len())..];

        Ok(Parsed { headers, body })
    } else if let Some(slice) = text.strip_prefix(line_pattern_crlf) {
        let index_of_ending_line = slice
            .find(line_pattern_crlf)
            .ok_or(FrontMatterError::MissingEndingLine)?;
        let headers = serde_yaml::from_str(&slice[..index_of_ending_line])
            .map_err(FrontMatterError::InvalidYaml)?;
        let body = &slice[(index_of_ending_line + line_pattern_crlf.len())..];

        Ok(Parsed { headers, body })
    } else {
        return Err(FrontMatterError::MissingBeginningLine);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct Headers {
        title: String,
    }

    #[test]
    fn parse_with_missing_beginning_line() {
        let text = "";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(FrontMatterError::MissingBeginningLine)));
    }

    #[test]
    fn parse_with_missing_ending_line() {
        let text = "---\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(FrontMatterError::MissingEndingLine)));
    }

    #[test]
    fn parse_with_missing_ending_line_crlf() {
        let text = "---\r\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(FrontMatterError::MissingEndingLine)));
    }

    #[test]
    fn parse_with_empty_frontmatter() {
        let text = "---\n---\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(FrontMatterError::InvalidYaml(_))));
    }

    #[test]
    fn parse_with_empty_frontmatter_crlf() {
        let text = "---\r\n---\r\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(FrontMatterError::InvalidYaml(_))));
    }

    #[test]
    fn parse_with_missing_known_field() {
        let text = "---\ntttttitle: dummy_title\n---\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(FrontMatterError::InvalidYaml(_))));
    }

    #[test]
    fn parse_with_missing_known_field_crlf() {
        let text = "---\r\nttttitle: dummy_title\r\n---\r\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Err(FrontMatterError::InvalidYaml(_))));
    }

    #[test]
    fn parse_with_unknown_field() {
        let text = "---\ndate: 2000-01-01\ntitle: dummy_title\n---\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn parse_with_unknown_field_crlf() {
        let text = "---\r\ndate: 2000-01-01\r\ntitle: dummy_title\r\n---\r\n";
        let result = parse::<Headers>(text);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn parse_with_empty_known_field() {
        let text = "---\ntitle:\n---\n";
        let result = parse::<Headers>(text).unwrap();
        
        assert_eq!(result.headers.title, "");
        assert_eq!(result.body, ""); 
    }
    
    #[test]
    fn parse_with_empty_known_field_crlf() {
        let text = "---\r\ntitle:\r\n---\r\n";
        let result = parse::<Headers>(text).unwrap();

        assert_eq!(result.headers.title, "");
        assert_eq!(result.body, "");
    }

    #[test]
    fn parse_with_valid_frontmatter() {
        let text = "---\ntitle: dummy_title---\ndummy_body";
        let result = parse::<Headers>(text).unwrap();
        assert_eq!(result.headers.title, "dummy_title");
        assert_eq!(result.body, "dummy_body");
    }
    #[test]
    fn parse_with_valid_frontmatter_crlf() {
        let text = "---\r\ntitle: dummy_title---\r\ndummy_body";
        let result = parse::<Headers>(text).unwrap();
        assert_eq!(result.headers.title, "dummy_title");
        assert_eq!(result.body, "dummy_body");
    }
}
