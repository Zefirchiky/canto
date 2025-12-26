use derive_more::{Deref, DerefMut};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenCreationError {
    #[error("Token contains space")]
    ContainsSpace,
    #[error("Token is empty")]
    Empty,
    #[error("")]
    Other,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deref, DerefMut)]
pub struct Token(String);

impl Token {
    pub fn new(tok: impl Into<String>) -> Token {
        let tok = tok.into();
        assert!(!tok.is_empty(), "Token {tok} is empty");
        assert!(!tok.contains(' '), "Token {tok} contains space(s)");
        Self(tok)
    }

    pub fn try_new(tok: impl Into<String>) -> Result<Token, TokenCreationError> {
        let tok = tok.into();
        if tok.is_empty() {
            return Err(TokenCreationError::Empty);
        } else if tok.contains(' ') {
            return Err(TokenCreationError::ContainsSpace);
        }
        Ok(Self(tok))
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
