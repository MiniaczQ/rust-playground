use async_trait::async_trait;

pub trait Event {
    type Context;
    type Output;
}

#[async_trait]
pub trait EventDispatchStrategy<E: Event> {
    async fn dispatch(&self, event: &E) -> E::Output;
}
