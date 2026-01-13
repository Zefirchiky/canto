use std::borrow::Cow;

use canto::{CantoContext, Context, FallbackWord, Word, WordParser};
use derive_more::Display;

#[derive(Debug, Default)]
struct BannedWordsContext {
    canto_context: CantoContext,
    pub words: Vec<String>,
}

impl Context for BannedWordsContext {
    fn get_canto_context(&self) -> &CantoContext {
        &self.canto_context
    }

    fn get_mut_canto_context(&mut self) -> &mut CantoContext {
        &mut self.canto_context
    }
}

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

    fn raw_text(&self, _ctx: &mut BannedWordsContext) -> Cow<'_, str> {
        Cow::Borrowed(&self.text)
    }

    fn text(&self, _ctx: &mut BannedWordsContext) -> Cow<'_, str> {
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
        words: vec!["this".into()],
        ..Default::default()
    };

    assert_eq!(parser.parse("this", &mut ctx)[0].raw_text(&mut ctx), "");
    assert_eq!(parser.parse("that", &mut ctx)[0].raw_text(&mut ctx), "that");
}