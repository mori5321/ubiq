use serde;
use serde_yaml;

#[derive(Debug)]
pub enum FrontMatterError {
    MissingBeginningLine,
    MissingEndingLine,
    InvalidYaml(serde_yaml::Error),
}

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
    fn parse_valid_frontmatter() {
        let input = r#"---
title: Hello World
---
This is John
"#;

        let Parsed { headers, body } = parse::<Headers>(input).unwrap();

        assert_eq!(headers.title, "Hello World");
        assert_eq!(body, "This is John\n");
    }

    // TODO: write invalid cases
}
