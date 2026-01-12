use canto::{Context, Novel};

#[derive(Debug, Default)]
struct SomeContext {
    _words: &'static [&'static str],
}

impl Context for SomeContext {}

#[test]
fn custom_context_novel_init() {
    let _novel = Novel::<SomeContext>::default();
}

#[test]
fn custom_context_push_dot() {
    
}