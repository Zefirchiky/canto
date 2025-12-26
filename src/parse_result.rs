use crate::Token;

#[derive(Debug)]
pub enum ParseResult<T> {
    Matched(T),
    Partial(Option<Token>, T, Option<Token>),
    NoMatch(Token),
}
