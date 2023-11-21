use async_trait::async_trait;

use super::event::Event;

#[async_trait]
pub trait AsyncDispatchStrategy<E: Event> {
    async fn dispatch(&self, event: &E) -> E::Output;
}
