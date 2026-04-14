use crate::domain::tokenizer::Token;

#[derive(Default)]
pub struct TokenFixture {
    tokens: Vec<Token>,
}

impl TokenFixture {
    pub fn tag(mut self, tag: &str) -> Self {
        self.tokens.push(Token::Tag(tag.to_owned()));
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.tokens.push(Token::Text(text.to_owned()));
        self
    }

    pub fn meta(mut self, key: &str, value: &str) -> Self {
        self.tokens.push(Token::Meta {
            key: key.to_owned(),
            value: value.to_owned(),
        });
        self
    }

    pub fn build(self) -> Vec<Token> {
        self.tokens
    }
}
