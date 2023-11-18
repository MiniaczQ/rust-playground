pub mod event;
pub mod listener;
pub mod storage;

#[cfg(test)]
mod tests {
    use super::{
        event::{Event, EventDispatchStrategy},
        listener::EventHandler,
        storage::EventHandlerStorage,
    };

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
                handler.handle(&mut ctx, event);
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
