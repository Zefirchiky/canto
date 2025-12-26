#[cfg(not(feature = "fxhash"))]
use std::collections::HashMap;

#[cfg(feature = "fxhash")]
use fxhash::FxHashMap;

use crate::{ParseResult, Priority, Token, Word, Words, word::Normal};

type WordConstructor = Box<dyn Fn(Token) -> ParseResult<Box<dyn Word>>>;

pub struct WordParser {
    #[cfg(not(feature = "fxhash"))]
    parsers: HashMap<Priority, Vec<WordConstructor>>,
    #[cfg(feature = "fxhash")]
    parsers: FxHashMap<Priority, Vec<WordConstructor>>,
}

impl WordParser {
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "fxhash"))]
            parsers: HashMap::new(),
            #[cfg(feature = "fxhash")]
            parsers: FxHashMap::default(),
        }
    }

    pub fn register<W: Word + 'static>(&mut self) {
        let priority = W::priority();
        let constructor: WordConstructor = Box::new(|token| match W::try_from_token(token) {
            ParseResult::Matched(word) => ParseResult::Matched(Box::new(word) as Box<dyn Word>),
            ParseResult::Partial(pre, word, post) => {
                ParseResult::Partial(pre, Box::new(word) as Box<dyn Word>, post)
            }
            ParseResult::NoMatch(tok) => ParseResult::NoMatch(tok),
        });

        self.parsers
            .entry(priority)
            .or_insert_with(Vec::new)
            .push(constructor);
    }

    pub fn parse(&self, token: impl Into<Token>) -> Words {
        let token = token.into();
        let mut words = Vec::with_capacity(10);
        let mut to_parse = vec![token];

        while let Some(current) = to_parse.pop() {
            let mut current_words = Vec::new();
            for priority in Priority::list() {
                if let Some(constructors) = self.parsers.get(&priority) {
                    for constructor in constructors {
                        match constructor(current.clone()) {
                            ParseResult::Matched(word) => {
                                current_words.push(word);
                                break;
                            }
                            ParseResult::Partial(pre, word, post) => {
                                if let Some(pre) = pre {
                                    current_words.extend(self.parse(pre).0);
                                }
                                current_words.push(word);
                                if let Some(post) = post {
                                    current_words.extend(self.parse(post).0);
                                }
                                break;
                            }
                            ParseResult::NoMatch(_) => continue,
                        }
                    }
                }
            }

            if current_words.is_empty() {
                current_words.push(Box::new(Normal::new(current.to_string())));
            }
            words.append(&mut current_words);
        }

        words.into()
    }
}

#[cfg(test)]
mod words_parsing {
    use crate::{
        WordParser,
        default_words::{Exclamation, QuestionMark},
    };

    #[test]
    fn normal() {
        let parser = WordParser::new();
        assert_eq!(parser.parse("dis")[0].raw_text(), "dis");
    }

    #[test]
    fn normal_with_exclamation() {
        let mut parser = WordParser::new();
        parser.register::<Exclamation>();
        let res = parser.parse("dis!");
        assert_eq!(res[0].raw_text(), "dis");
        assert_eq!(res[1].raw_text(), "!");
    }

    #[test]
    fn complex() {
        let mut parser = WordParser::new();
        parser.register::<Exclamation>();
        parser.register::<QuestionMark>();
        let res = parser.parse("?dis!das");
        assert_eq!(res[0].raw_text(), "?");
        assert_eq!(res[1].raw_text(), "dis");
        assert_eq!(res[2].raw_text(), "!");
        assert_eq!(res[3].raw_text(), "das");
    }
}
