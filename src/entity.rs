use std::collections::HashMap;

use raylib::prelude::*;

pub trait Entity {
    fn draw<'a>(&self, d: RaylibDrawHandle<'a>) -> RaylibDrawHandle<'a>;
    fn is_finished(&self) -> bool;
    fn set_pos(&mut self, pos: Vector2);
}

pub struct EntityManager<T: Entity> {
    entities: HashMap<usize, T>,
    last_id: usize,
}

impl<T: Entity> EntityManager<T> {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            last_id: 0,
        }
    }

    pub fn insert(&mut self, entity: T) -> usize {
        self.last_id += 1;
        self.entities.insert(self.last_id, entity);
        self.last_id
    }

    // Unused
    // pub fn remove(&mut self, id: usize) {
    //     self.entities.remove(&id);
    // }

    pub fn update<F>(&mut self, mut closure: F)
    where
        F: FnMut(&mut T),
    {
        for entity in self.entities.values_mut() {
            closure(entity);
        }

        self.entities.retain(|_, entity| !entity.is_finished());
    }

    pub fn draw<'d>(&self, mut d: RaylibDrawHandle<'d>) -> RaylibDrawHandle<'d> {
        for entity in self.entities.values() {
            d = entity.draw(d);
        }
        d
    }

    pub fn set_pos(&mut self, id: usize, pos: Vector2) {
        if let Some(entity) = self.entities.get_mut(&id) {
            entity.set_pos(pos);
        }
    }

    pub fn is_finished(&self, id: usize) -> bool {
        self.entities
            .get(&id)
            .map_or(true, |entity| entity.is_finished())
    }
}
