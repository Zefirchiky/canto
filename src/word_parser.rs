#[cfg(not(feature = "fxhash"))]
use std::collections::HashMap;

#[cfg(feature = "fxhash")]
use fxhash::FxHashMap;

use crate::{Context, EmptyContext, FallbackWord, ParseResult, Priority, Token, Word, Words, word::Normal};

type WordConstructor<C> = fn(Token, &mut C) -> ParseResult<Box<dyn Word<C>>>;

pub struct WordParser<C = EmptyContext, F = Normal>
where
    C: Context,
    F: FallbackWord<C> + 'static
{
    #[cfg(not(feature = "fxhash"))]
    parsers: HashMap<Priority, Vec<WordConstructor<C>>>,
    #[cfg(feature = "fxhash")]
    parsers: FxHashMap<Priority, Vec<WordConstructor<C>>>,
    fallback: fn(Token, &mut C) -> F
}

impl<C: Context, F: FallbackWord<C>> WordParser<C, F> {
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "fxhash"))]
            parsers: HashMap::new(),
            #[cfg(feature = "fxhash")]
            parsers: FxHashMap::default(),
            fallback: |tok, ctx| F::from_token(tok, ctx),
        }
    }

    pub fn register<W: Word<C> + 'static>(&mut self) {
        let priority = W::priority();
        let constructor = |token, ctx: &mut C| {
            match W::try_from_token(token, ctx) {
                ParseResult::Matched(word) => ParseResult::Matched(Box::new(word) as Box<dyn Word<C>>),
                ParseResult::Partial(pre, word, post) => {
                    ParseResult::Partial(pre, Box::new(word) as Box<dyn Word<C>>, post)
                }
                ParseResult::NoMatch(tok) => ParseResult::NoMatch(tok),
            }
        };

        self.parsers
            .entry(priority)
            .or_insert_with(Vec::new)
            .push(constructor);
    }

    pub fn parse(&self, token: impl Into<Token>, ctx: &mut C) -> Words<C> {
        let token = token.into();
        let mut words = Vec::with_capacity(10);
        let mut to_parse = vec![token];

        while let Some(current) = to_parse.pop() {
            let mut current_words = Vec::new();
            for priority in Priority::list() {
                if let Some(constructors) = self.parsers.get(&priority) {
                    for constructor in constructors {
                        match constructor(current.clone(), ctx) {
                            ParseResult::Matched(word) => {
                                current_words.push(word);
                                break;
                            }
                            ParseResult::Partial(pre, word, post) => {
                                if let Some(pre) = pre {
                                    current_words.extend(self.parse(pre, ctx).0);
                                }
                                current_words.push(word);
                                if let Some(post) = post {
                                    current_words.extend(self.parse(post, ctx).0);
                                }
                                break;
                            }
                            ParseResult::NoMatch(_) => continue,
                        }
                    }
                }
            }

            if current_words.is_empty() {
                current_words.push(Box::new((self.fallback)(current, ctx)));
            }
            words.append(&mut current_words);
        }

        words.into()
    }
}

#[cfg(test)]
mod words_parsing {
    use crate::{
        EmptyContext, WordParser, default_words::{Exclamation, QuestionMark}
    };

    #[test]
    fn normal() {
        let parser: WordParser = WordParser::new();
        assert_eq!(parser.parse("dis", &mut EmptyContext)[0].raw_text(), "dis");
    }

    #[test]
    fn normal_with_exclamation() {
        let mut parser: WordParser = WordParser::new();
        parser.register::<Exclamation>();
        let res = parser.parse("dis!", &mut EmptyContext);
        assert_eq!(res[0].raw_text(), "dis");
        assert_eq!(res[1].raw_text(), "!");
    }

    #[test]
    fn complex() {
        let mut parser: WordParser = WordParser::new();
        parser.register::<Exclamation>();
        parser.register::<QuestionMark>();
        let res = parser.parse("?dis!das", &mut EmptyContext);
        assert_eq!(res[0].raw_text(), "?");
        assert_eq!(res[1].raw_text(), "dis");
        assert_eq!(res[2].raw_text(), "!");
        assert_eq!(res[3].raw_text(), "das");
    }
}
