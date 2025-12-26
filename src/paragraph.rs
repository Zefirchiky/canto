use derive_more::{Deref, DerefMut, From};

use crate::Token;

#[derive(Debug, Default, Deref, DerefMut, From)]
pub struct Paragraphs(Vec<Paragraph>);

#[derive(Debug, Default, Deref, DerefMut)]
pub struct Paragraph {
    elements: Vec<Token>,
}
