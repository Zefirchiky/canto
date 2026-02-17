use std::rc::Rc;

use canto_mh::{CantoContext, Context, Novel};

#[derive(Debug, Default)]
struct SomeContext {
    canto_context: Rc<CantoContext>,
    _words: &'static [&'static str],
}

impl Context for SomeContext {
    fn get_canto_context(&self) -> Rc<CantoContext> {
        self.canto_context.clone()
    }

    // fn get_mut_canto_context(&mut self) -> &mut CantoContext {
    //     &mut self.canto_context
    // }
}

#[test]
fn custom_context_novel_init() {
    let _novel = Novel::<SomeContext>::default();
}

#[test]
fn custom_context_push_dot() {}
