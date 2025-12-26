use std::fmt::{Debug, Display};

use derive_more::Display;

use crate::{ParseResult, Priority, Token, Word};

// #[derive(Debug, Display)]
pub trait Punctuation: Debug + Display + From<Token> {
    /// The character this punctuation represents
    fn text() -> &'static str
    where
        Self: Sized;
}

// Blanket implementation of Word for all Punctuation types
impl<P: Punctuation + 'static> Word for P {
    fn try_from_token(mut token: Token) -> ParseResult<Self>
    where
        Self: Sized,
    {
        if token.len() == Self::text().len() {
            dbg!(Self::text());
            ParseResult::Matched(token.into())
        } else {
            // Token has more - need to split
            let text = Self::text();

            if let Some(pos) = token.find(text) {
                let rest = token.split_off(pos); // token becomes post, while rest becomes pre + Self::text()
                let pre = token;
                let (matched, post) = rest.split_at(text.len());

                let pre_token = if pre.is_empty() {
                    None
                } else {
                    Some(Token::from(pre))
                };
                let post_token = if post.is_empty() {
                    None
                } else {
                    Some(Token::from(post))
                };

                ParseResult::Partial(pre_token, Token::new(matched).into(), post_token)
            } else {
                ParseResult::NoMatch(token)
            }
        }
    }

    fn priority() -> Priority
    where
        Self: Sized,
    {
        Priority::Highest
    }

    fn raw_text(&self) -> &str {
        Self::text()
    }
}

#[derive(Debug, Display)]
#[display("!")]
pub struct Exclamation;

impl Punctuation for Exclamation {
    fn text() -> &'static str
    where
        Self: Sized,
    {
        "!"
    }
}

impl From<Token> for Exclamation {
    fn from(_value: Token) -> Self {
        Self
    }
}

#[derive(Debug, Display)]
#[display("?")]
pub struct QuestionMark;

impl Punctuation for QuestionMark {
    fn text() -> &'static str
    where
        Self: Sized,
    {
        "?"
    }
}

impl From<Token> for QuestionMark {
    fn from(_value: Token) -> Self {
        Self
    }
}

#[cfg(test)]
mod punctuation_parsing {
    use crate::{ParseResult, Token, Word, default_words::Exclamation};

    #[test]
    fn right_exclamation() {
        if let ParseResult::Matched(mark) = Exclamation::try_from_token(Token::new("!")) {
            assert_eq!(mark.raw_text(), "!");
        } else {
            panic!()
        }
    }

    #[test]
    fn first_part_right_exclamation() {
        if let ParseResult::Partial(Some(dis), mark, None) =
            Exclamation::try_from_token(Token::new("dis!"))
        {
            assert_eq!(mark.raw_text(), "!", "Mark is not `!`");
            assert_eq!(dis.as_str(), "dis", "Rest is not `dis`");
        } else {
            panic!()
        }
    }

    #[test]
    fn second_part_right_exclamation() {
        if let ParseResult::Partial(None, mark, Some(dis)) =
            Exclamation::try_from_token(Token::new("!dis"))
        {
            assert_eq!(mark.raw_text(), "!", "Mark is not `!`");
            assert_eq!(dis.as_str(), "dis", "Rest is not `dis`");
        } else {
            panic!()
        }
    }

    #[test]
    fn both_part_right_exclamation() {
        if let ParseResult::Partial(Some(dis), mark, Some(dis2)) =
            Exclamation::try_from_token(Token::new("dis!dis2"))
        {
            assert_eq!(mark.raw_text(), "!", "Mark is not `!`");
            assert_eq!(dis.as_str(), "dis", "Rest is not `dis`");
            assert_eq!(dis2.as_str(), "dis2", "Rest is not `dis`");
        } else {
            panic!()
        }
    }

    #[test]
    fn no_match_exclamation() {
        assert!(matches!(
            Exclamation::try_from_token(Token::new("dis")),
            ParseResult::NoMatch(_token)
        ))
    }
}
