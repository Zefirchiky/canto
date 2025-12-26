use std::fmt::{Debug, Display};

use derive_more::{Deref, DerefMut, Display, From};

use crate::{ParseResult, Priority, Token};

#[derive(Debug, Default, Deref, DerefMut, From)]
pub struct Words(pub Vec<Box<dyn Word>>);

pub trait Word: Debug + Display {
    /// Can word be multi-token?
    fn is_multi_token() -> bool
    where
        Self: Sized,
    {
        false
    }

    fn try_from_token(token: Token) -> ParseResult<Self>
    where
        Self: Sized;

    /// All Words of the same `Priority` will be grouped, and order is not guaranteed.
    ///
    /// Words of higher `Priority` will be tried first, and if none passes, lower `Priority` will be tried.
    /// `Normal` Word is a fallback.
    fn priority() -> Priority
    where
        Self: Sized,
    {
        Priority::Mid
    }

    fn raw_text(&self) -> &str;
}

/// The fallback
#[derive(Debug, Display)]
pub struct Normal {
    text: String,
}

impl Normal {
    pub fn new(tok: impl Into<String>) -> Self {
        Self { text: tok.into() }
    }
}

impl Word for Normal {
    fn try_from_token(token: crate::Token) -> ParseResult<Self>
    where
        Self: Sized,
    {
        ParseResult::Matched(Self {
            text: token.to_string(),
        })
    }

    fn priority() -> crate::Priority
    where
        Self: Sized,
    {
        crate::Priority::Lowest
    }

    fn raw_text(&self) -> &str {
        &self.text
    }
}
