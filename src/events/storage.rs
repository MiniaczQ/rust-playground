use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use super::{event::Event, listener::AsyncListener};

#[derive(Debug, Default)]
pub struct AsyncListenerStorage {
    storage: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

type EventHandlerVec<E> = Vec<Box<dyn AsyncListener<E>>>;

impl AsyncListenerStorage {
    pub fn add<E: Event + 'static>(&mut self, handler: impl AsyncListener<E> + 'static) {
        let type_id = TypeId::of::<E>();
        let handlers = self
            .storage
            .entry(type_id)
            .or_insert_with(|| Box::<Vec<Box<dyn AsyncListener<E>>>>::default())
            .downcast_mut::<EventHandlerVec<E>>()
            .unwrap();
        handlers.push(Box::new(handler));
    }

    pub fn get<E: Event + 'static>(&self) -> &[Box<dyn AsyncListener<E>>] {
        let type_id = TypeId::of::<E>();
        self.storage
            .get(&type_id)
            .map(|handlers| {
                handlers.downcast_ref::<EventHandlerVec<E>>().unwrap()
                    as &[Box<dyn AsyncListener<E>>]
            })
            .unwrap_or(&[])
    }
}
