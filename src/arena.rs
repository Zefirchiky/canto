use std::{any::Any, collections::HashMap, marker::PhantomData};

use slotmap::{DefaultKey, SlotMap};

use crate::{Context, EmptyContext, Word};

pub trait ArenaBox<C: Context = EmptyContext>: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get(&self, key: DefaultKey) -> Option<&dyn Word<C>>;
    fn get_mut(&mut self, key: DefaultKey) -> Option<&mut dyn Word<C>>;
}

pub struct SimpleWordArena<C: Context, W: Word<C>> {
    arena: SlotMap<DefaultKey, W>,
    phant_data: PhantomData<C>,
}

impl<C: Context + 'static, W: Word<C> + 'static> ArenaBox<C> for SimpleWordArena<C, W> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get(&self, key: DefaultKey) -> Option<&dyn Word<C>> {
        self.arena.get(key).map(|w| w as &dyn Word<C>)
    }

    fn get_mut(&mut self, key: DefaultKey) -> Option<&mut dyn Word<C>> {
        self.arena.get_mut(key).map(|w| w as &mut dyn Word<C>)
    }
}

pub struct DedupWordArena<C: Context, W: Word<C>> {
    arena: SlotMap<DefaultKey, W>,
    key_map: HashMap<String, DefaultKey>,
    phant_data: PhantomData<C>,
}

impl<C: Context, W: Word<C>> DedupWordArena<C, W> {
    pub fn new() -> Self {
        Self {
            arena: SlotMap::new(),
            key_map: HashMap::new(),
            phant_data: PhantomData,
        }
    }

    pub fn insert(&mut self, word: W, ctx: &mut C) -> DefaultKey {
        let text = word.text(ctx).to_string();
        let key = self.arena.insert(word);
        self.key_map.insert(text, key.clone());
        key
    }
}

impl<C: Context + 'static, W: Word<C> + 'static> ArenaBox<C> for DedupWordArena<C, W> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get(&self, key: DefaultKey) -> Option<&dyn Word<C>> {
        self.arena.get(key).map(|w| w as &dyn Word<C>)
    }

    fn get_mut(&mut self, key: DefaultKey) -> Option<&mut dyn Word<C>> {
        self.arena.get_mut(key).map(|w| w as &mut dyn Word<C>)
    }
}
