#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Tag(String),
    Text(String),
    Meta { key: String, value: String },
}

pub fn parse_query(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    // tokenize with quote support
    let mut parts = Vec::new();

    for c in input.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    // classify tokens
    for part in parts {
        if let Some(tag) = part.strip_prefix("t:") {
            if !tag.is_empty() {
                tokens.push(Token::Tag(tag.to_string()));
            }
            continue;
        }

        if let Some((key, value)) = part.split_once(':') {
            if !key.is_empty() && !value.is_empty() {
                tokens.push(Token::Meta {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            }
            continue;
        }

        // plain text
        tokens.push(Token::Text(part));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_empty_tag_and_meta() {
        let result = parse_query("t: author: :value");

        assert_eq!(result, vec![]);
    }

    #[test]
    fn supports_quoted_values() {
        let result = parse_query(r#"author:"john doe""#);

        assert_eq!(
            result,
            vec![Token::Meta {
                key: "author".to_string(),
                value: "john doe".to_string(),
            }]
        );
    }

    #[test]
    fn full_mixed_query() {
        let result = parse_query(r#"t:rust tui author:"john doe""#);

        assert_eq!(
            result,
            vec![
                Token::Tag("rust".to_string()),
                Token::Text("tui".to_string()),
                Token::Meta {
                    key: "author".to_string(),
                    value: "john doe".to_string(),
                }
            ]
        );
    }
}
