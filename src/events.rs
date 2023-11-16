use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub trait EventDispatchStrategy<E: Event> {
    fn dispatch(&self, event: &E) -> E::Output;
}

pub trait Event {
    type Context;
    type Output;
}

pub trait EventHandler<E: Event> {
    fn handle(&self, ctx: &mut E::Context, event: &E);
}

#[derive(Debug, Default)]
pub struct EventHandlerStorage {
    storage: HashMap<TypeId, Box<dyn Any>>,
}

type EventHandgit add lerVec<E> = Vec<Box<dyn EventHandler<E>>>;

impl EventHandlerStorage {
    pub fn add<E: Event + 'static>(&mut self, handler: impl EventHandler<E> + 'static) {
        let type_id = TypeId::of::<E>();

        if self.storage.get(&type_id).is_none() {
            self.storage
                .insert(type_id, Box::new(EventHandlerVec::<E>::new()));
        }

        let handlers = self.storage.get_mut(&type_id).unwrap();
        let handlers = handlers.downcast_mut::<EventHandlerVec<E>>().unwrap();

        handlers.push(Box::new(handler));
    }

    pub fn get<E: Event + 'static>(&self) -> &[Box<dyn EventHandler<E>>] {
        let type_id = TypeId::of::<E>();

        let Some(handlers) = self.storage.get(&type_id) else {
            return &[];
        };

        let handlers = handlers.downcast_ref::<EventHandlerVec<E>>().unwrap();

        &handlers
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, EventDispatchStrategy, EventHandler, EventHandlerStorage};

    pub struct FooBarEvent {
        foo: String,
        bar: u64,
    }

    impl Event for FooBarEvent {
        type Context = String;
        type Output = String;
    }

    struct FooBarEventHandler;

    impl EventHandler<FooBarEvent> for FooBarEventHandler {
        fn handle(&self, ctx: &mut <FooBarEvent as Event>::Context, event: &FooBarEvent) {
            *ctx = format!("{}{}-{} ", ctx, event.bar, event.foo);
        }
    }

    impl EventDispatchStrategy<FooBarEvent> for EventHandlerStorage {
        fn dispatch(&self, event: &FooBarEvent) -> <FooBarEvent as Event>::Output {
            let handlers = self.get::<FooBarEvent>();
            let mut ctx: String = String::new();

            for handler in handlers {
                handler.handle(&mut ctx, &event);
            }

            ctx
        }
    }

    #[test]
    fn test() {
        let mut handlers = EventHandlerStorage::default();
        handlers.add(FooBarEventHandler);
        handlers.add(FooBarEventHandler);

        let result = handlers.dispatch(&FooBarEvent {
            foo: "a".to_owned(),
            bar: 7,
        });

        assert_eq!(result, "7-a 7-a ");
    }
}
