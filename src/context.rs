use std::sync::Arc;

use derive_more::Debug;
use spel_right::SpellChecker;

use crate::word::WordId;

pub trait Context: std::fmt::Debug + Default {
    fn get_canto_context(&self) -> &CantoContext;
    fn get_mut_canto_context(&mut self) -> &mut CantoContext;
}

#[derive(Debug, Default)]
pub struct CantoContext {
    #[debug(skip)]
    pub spell_checker: Arc<SpellChecker>,
    pub prev_word: Option<Box<dyn WordId>>,
}

#[derive(Debug, Default)]
pub struct EmptyContext {
    canto_context: CantoContext,
}

impl Context for EmptyContext {
    fn get_canto_context(&self) -> &CantoContext {
        &self.canto_context
    }

    fn get_mut_canto_context(&mut self) -> &mut CantoContext {
        &mut self.canto_context
    }
}
