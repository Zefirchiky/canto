use std::{rc::Rc, sync::Arc};

use derive_more::Debug;
use spel_right::SpellChecker;

use crate::word::WordId;

pub trait Context: std::fmt::Debug + Default {
    fn get_canto_context(&self) -> Rc<CantoContext>;
    // fn get_mut_canto_context(&mut self) -> Rc<&mut CantoContext>;
    // fn get_word(&self, id: WordId) -> Box<&dyn Word>;
}

#[derive(Debug, Default)]
pub struct CantoContext {
    #[debug(skip)]
    pub spell_checker: Arc<SpellChecker>,
    pub prev_word: Option<WordId>,
}

#[derive(Debug, Default)]
pub struct EmptyContext {
    canto_context: Rc<CantoContext>,
}

impl Context for EmptyContext {
    fn get_canto_context(&self) -> Rc<CantoContext> {
        self.canto_context.clone()
    }

    // fn get_mut_canto_context(&mut self) -> &mut CantoContext {
    //     &mut self.canto_context
    // }

    // fn get_word(&self, id: WordId) -> Box<&dyn Word> {

    // }
}
