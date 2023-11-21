//! Module for dependency injected events & listeners.
//!
//! Event consists of input, execution context and output.
//! Input is shared by all listeners, context can be shared or separated, output is the combined result of all listeners.
//! Strategy dictates how context is created and turned into output, as well as how handlers are executed.
//!
//! The module provides a generic-agnostic event handler storage.

pub mod dispatch;
pub mod event;
pub mod listener;
pub mod storage;

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::{
        dispatch::AsyncDispatchStrategy, event::Event, listener::AsyncListener,
        storage::AsyncListenerStorage,
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

    #[async_trait]
    impl AsyncListener<FooBarEvent> for FooBarEventHandler {
        async fn handle(&self, ctx: &mut <FooBarEvent as Event>::Context, event: &FooBarEvent) {
            *ctx = format!("{}{}-{} ", ctx, event.bar, event.foo);
        }
    }

    #[async_trait]
    impl AsyncDispatchStrategy for FooBarEvent {
        async fn dispatch<'a>(
            &self,
            handlers: impl Iterator<Item = &'a Box<dyn AsyncListener<Self>>> + Send + Sync,
        ) -> Self::Output
        where
            Self: 'a,
        {
            let mut ctx: String = String::new();

            for handler in handlers {
                handler.handle(&mut ctx, self).await;
            }

            ctx
        }
    }

    #[tokio::test]
    async fn test() {
        let mut storage = AsyncListenerStorage::default();
        storage.add(FooBarEventHandler);
        storage.add(FooBarEventHandler);

        let event = FooBarEvent {
            foo: "a".to_owned(),
            bar: 7,
        };
        let handlers = storage.get::<FooBarEvent>();
        let result = event.dispatch(handlers.iter()).await;

        assert_eq!(result, "7-a 7-a ");
    }
}
