use std::borrow::Cow;

use canto::{Context, FallbackWord, Word, WordParser};
use derive_more::Display;

#[derive(Debug, Default)]
struct BannedWordsContext {
    pub words: Vec<String>,
}

impl Context for BannedWordsContext {}

#[derive(Debug, Display, Default)]
struct SfwWord {
    text: String,
}

impl Word<BannedWordsContext> for SfwWord {
    fn try_from_token(token: canto::Token, ctx: &mut BannedWordsContext) -> canto::ParseResult<Self>
        where
            Self: Sized {
        canto::ParseResult::Matched(Self::from_token(token, ctx))
    }

    fn raw_text(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.text)
    }
}

impl FallbackWord<BannedWordsContext> for SfwWord {
    fn from_token(token: canto::Token, ctx: &mut BannedWordsContext) -> Self
        where
            Self: Sized {
        if ctx.words.contains(&token) {
            Self {
                text: "".into()
            }
        } else {
            Self {
                text: token.to_string()
            }
        }
    }
}

#[test]
fn fallback_word() {
    let parser = WordParser::<BannedWordsContext, SfwWord>::new();
    let mut ctx = BannedWordsContext {
        words: vec!["this".into()]
    };

    assert_eq!(parser.parse("this", &mut ctx)[0].raw_text(), "");
    assert_eq!(parser.parse("that", &mut ctx)[0].raw_text(), "that");
}