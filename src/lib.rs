mod novel;
mod paragraph;
mod parse_result;
mod priority;
mod token;
pub mod word;
mod word_parser;

pub mod default_words;

pub use novel::Novel;
pub use paragraph::{Paragraph, Paragraphs};
pub use parse_result::ParseResult;
pub use priority::Priority;
pub use token::Token;
pub use word::{Word, Words};
pub use word_parser::WordParser;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
