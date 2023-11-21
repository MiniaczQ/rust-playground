use async_trait::async_trait;

use super::event::Event;

#[async_trait]
pub trait AsyncListener<E: Event>: Send + Sync {
    async fn handle(&self, ctx: &mut E::Context, event: &E);
}
