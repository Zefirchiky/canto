use crate::{Context, EmptyContext, Paragraphs};

#[derive(Debug, Default)]
pub struct Novel<C: Context = EmptyContext> {
    pub paragraphs: Paragraphs<C>,
}
