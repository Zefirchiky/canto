use derive_more::{Deref, DerefMut, From};

use crate::{Context, EmptyContext, Words};

#[derive(Debug, Default, Deref, DerefMut, From)]
pub struct Paragraphs<C: Context = EmptyContext>(Vec<Paragraph<C>>);

#[derive(Debug, Default, Deref, DerefMut)]
pub struct Paragraph<C: Context = EmptyContext> {
    elements: Words<C>,
}

impl Paragraph {
    pub fn new() -> Paragraph {
        Self {
            elements: Words::default(),
        }
    }
}
