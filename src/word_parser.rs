use std::cell::RefCell;
#[cfg(not(feature = "fxhash"))]
use std::collections::HashMap;

#[cfg(feature = "fxhash")]
use fxhash::FxHashMap;

use crate::{
    ArenaBox, Context, DedupWordArena, EmptyContext, FallbackWord, ParseResult, Priority, Token,
    Word, Words,
    word::{CompletionStatus, Normal},
};

type WordConstructor<C> = fn(Token, &mut C) -> ParseResult<Box<dyn Word<C>>>;

pub struct WordParser<C = EmptyContext, F = Normal>
where
    C: Context,
    F: FallbackWord<C> + 'static,
{
    #[cfg(not(feature = "fxhash"))]
    parsers: HashMap<Priority, Vec<WordConstructor<C>>>,
    #[cfg(feature = "fxhash")]
    parsers: FxHashMap<Priority, Vec<WordConstructor<C>>>,

    arenas: RefCell<Vec<Box<dyn ArenaBox<C>>>>, // FIXME: Transfer arenas elsewhere. They are data storages, not fit for parser

    fallback: fn(Token, &mut C) -> F,
}

impl<C: Context + 'static, F: FallbackWord<C>> WordParser<C, F> {
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "fxhash"))]
            parsers: HashMap::new(),
            #[cfg(feature = "fxhash")]
            parsers: FxHashMap::default(),
            arenas: RefCell::new(vec![]),
            fallback: |tok, ctx| F::from_token(tok, ctx),
        }
    }

    pub fn register<W: Word<C> + 'static>(&mut self) -> usize {
        let priority = W::priority();
        let constructor = |token, ctx: &mut C| match W::try_from_token(token, ctx) {
            ParseResult::Matched(word) => ParseResult::Matched(Box::new(word) as Box<dyn Word<C>>),
            ParseResult::Partial(pre, word, post) => {
                ParseResult::Partial(pre, Box::new(word) as Box<dyn Word<C>>, post)
            }
            ParseResult::NoMatch(tok) => ParseResult::NoMatch(tok),
        };

        self.parsers
            .entry(priority)
            .or_insert_with(Vec::new)
            .push(constructor);

        self.arenas
            .borrow_mut()
            .push(Box::new(DedupWordArena::<C, W>::new()) as Box<dyn ArenaBox<C>>);
        // .expect("2 of the same TypeId shouldn't be possible");
        self.arenas.borrow().len() - 1 // Arena id
    }

    pub fn parse(&self, token: impl Into<Token>, ctx: &mut C) -> Words<C> {
        // ! Complex issue:
        // Multi-token words can forever request new tokens.
        // Possible solution:
        //      Parse higher priority parsers first, and pass token to previous word at it's priority.
        // What if multi-token word requires other tokens to be finished, and higher priority parser
        // take those tokens
        // Possible solution:
        //      Words can give 'need token to finish' signal, where they will be prioritized until finished.
        // Back to problem 1 for those words
        // Possible solution
        //      Double pass, give tokens to both, word, and higher priority parsers, and keep track of them.
        //      This is complicated tho.

        // Current strategy:
        // Parse higher priorities first, previous token will be incomplete

        let mut token = token.into();
        let mut words = Vec::with_capacity(10);
        let canto = ctx.get_canto_context();

        for priority in Priority::list() {
            if let Some(word_id) = canto.prev_word {
                if let Some(arena) = self.arenas.borrow_mut().get_mut(word_id.arena) {
                    if let Some(word) = arena.get_mut(word_id.id) {
                        if word.completion_status() == CompletionStatus::Incomplete {
                            if let Some(tok) = word.push_token(token, ctx) {
                                token = tok
                            } else {
                                return vec![].into();
                            }
                        }
                    }
                }
            }
            if let Some(constructors) = self.parsers.get(&priority) {
                for constructor in constructors {
                    match constructor(token.clone(), ctx) {
                        ParseResult::Matched(word) => {
                            words.push(word);
                            break;
                        }
                        ParseResult::Partial(pre, word, post) => {
                            if let Some(pre) = pre {
                                words.extend(self.parse(pre, ctx).0);
                            }
                            words.push(word);
                            if let Some(post) = post {
                                words.extend(self.parse(post, ctx).0);
                            }
                            break;
                        }
                        ParseResult::NoMatch(_) => continue,
                    }
                }
            }
        }

        if words.is_empty() {
            words.push(Box::new((self.fallback)(token, ctx)));
        }

        words.into()
    }
}

#[cfg(test)]
mod words_parsing {
    use crate::{
        EmptyContext, WordParser,
        default_words::{Exclamation, QuestionMark},
    };

    #[test]
    fn normal() {
        let parser: WordParser = WordParser::new();
        assert_eq!(
            parser.parse("dis", &mut EmptyContext::default())[0]
                .raw_text(&mut EmptyContext::default()),
            "dis"
        );
    }

    #[test]
    fn normal_with_exclamation() {
        let mut parser: WordParser = WordParser::new();
        parser.register::<Exclamation>();
        let mut ctx = EmptyContext::default();
        let res = parser.parse("dis!", &mut ctx);
        assert_eq!(res[0].raw_text(&mut ctx), "dis");
        assert_eq!(res[1].raw_text(&mut ctx), "!");
    }

    #[test]
    fn complex() {
        let mut parser: WordParser = WordParser::new();
        parser.register::<Exclamation>();
        parser.register::<QuestionMark>();
        let mut ctx = EmptyContext::default();
        let res = parser.parse("?dis!das", &mut ctx);
        assert_eq!(res[0].raw_text(&mut ctx), "?");
        assert_eq!(res[1].raw_text(&mut ctx), "dis");
        assert_eq!(res[2].raw_text(&mut ctx), "!");
        assert_eq!(res[3].raw_text(&mut ctx), "das");
    }
}
