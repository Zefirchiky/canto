use std::fmt::Debug;

pub trait Context: Debug + Default {}

#[derive(Debug, Default)]
pub struct EmptyContext;

impl Context for EmptyContext {}
