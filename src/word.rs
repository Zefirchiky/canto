use std::{borrow::Cow, fmt::Debug};

use derive_more::{Deref, DerefMut, From};

use crate::{Context, EmptyContext, ParseResult, Priority, Token};

pub trait WordId: Debug {}

#[derive(Debug, Default, Deref, DerefMut, From)]
pub struct Words<C: Context = EmptyContext>(pub Vec<Box<dyn Word<C>>>);

pub trait Word<C: Context = EmptyContext>: Debug {
    /// Can word be multi-token?
    fn is_multi_token() -> bool
    where
        Self: Sized,
    {
        false
    }

    fn try_from_token(token: Token, _ctx: &mut C) -> ParseResult<Self>
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

    fn raw_text(&self, _ctx: &mut C) -> Cow<'_, str>;
    fn text(&self, _ctx: &mut C) -> Cow<'_, str>;
}

pub trait FallbackWord<C: Context = EmptyContext>: Word<C> {
    fn from_token(token: Token, _ctx: &mut C) -> Self
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum NormalWordId {
    Correct(spel_right::WordId),
    Incorrect(String),  // FIXME: Should be Uuid to arena
}

impl WordId for NormalWordId {}

/// The fallback
#[derive(Debug)]
pub struct Normal {
    id: NormalWordId,
}

impl<C: Context> Word<C> for Normal {
    fn try_from_token(token: Token, ctx: &mut C) -> ParseResult<Self>
        where
            Self: Sized {
        ParseResult::Matched(Self::from_token(token, ctx))
    }

    fn priority() -> Priority
        where
            Self: Sized, {
        Priority::Lowest
    }

    fn raw_text(&self, ctx: &mut C) -> Cow<'_, str> {
        match &self.id {
            NormalWordId::Correct(id) => Cow::Owned(match ctx.get_canto_context().spell_checker.get(*id) {
                Some(word) => word.to_string(),
                None => "".into(),
            }),
            NormalWordId::Incorrect(word) => Cow::Borrowed(&word)
        }
    }
        
    fn text(&self, ctx: &mut C) -> Cow<'_, str> {
        match &self.id {
            NormalWordId::Correct(id) => Cow::Owned(match ctx.get_canto_context().spell_checker.get(*id) {  // FIXME: Need to deal with lifetimes, to not copy the &str
                Some(word) => word.to_string(),
                None => "".into(),
            }),
            NormalWordId::Incorrect(word) => Cow::Borrowed(&word)
        }
    }
}

impl<C: Context> FallbackWord<C> for Normal {
    fn from_token(token: crate::Token, ctx: &mut C) -> Self
    where
        Self: Sized,
    {
        let canto = ctx.get_canto_context();
        if let Some(id) = canto.spell_checker.find(&token) {
            Self {
                id: NormalWordId::Correct(id),
            }
        } else {
            Self {
                id: NormalWordId::Incorrect(token.to_string())
            }
        }
    }
}
