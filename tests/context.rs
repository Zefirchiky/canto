use canto::{CantoContext, Context, Novel};

#[derive(Debug, Default)]
struct SomeContext {
    canto_context: CantoContext,
    _words: &'static [&'static str],
}

impl Context for SomeContext {
    fn get_canto_context(&self) -> &CantoContext {
        &self.canto_context
    }

    fn get_mut_canto_context(&mut self) -> &mut CantoContext {
        &mut self.canto_context
    }
}

#[test]
fn custom_context_novel_init() {
    let _novel = Novel::<SomeContext>::default();
}

#[test]
fn custom_context_push_dot() {
    
}