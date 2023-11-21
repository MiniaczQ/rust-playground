use async_trait::async_trait;

use super::{event::Event, listener::AsyncListener};

#[async_trait]
pub trait AsyncDispatchStrategy: Event {
    async fn dispatch<'a>(
        &self,
        handlers: impl Iterator<Item = &'a Box<dyn AsyncListener<Self>>> + Send + Sync,
    ) -> Self::Output
    where
        Self: 'a;
}
