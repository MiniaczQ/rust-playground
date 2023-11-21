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

        if self.storage.get(&type_id).is_none() {
            self.storage
                .insert(type_id, Box::<Vec<Box<dyn AsyncListener<E>>>>::default());
        }

        let handlers = self.storage.get_mut(&type_id).unwrap();
        let handlers = handlers.downcast_mut::<EventHandlerVec<E>>().unwrap();

        handlers.push(Box::new(handler));
    }

    pub fn get<E: Event + 'static>(&self) -> &[Box<dyn AsyncListener<E>>] {
        let type_id = TypeId::of::<E>();

        let Some(handlers) = self.storage.get(&type_id) else {
            return &[];
        };

        let handlers = handlers.downcast_ref::<EventHandlerVec<E>>().unwrap();

        handlers
    }
}
