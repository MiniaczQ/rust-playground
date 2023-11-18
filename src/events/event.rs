pub trait Event {
    type Context;
    type Output;
}

pub trait EventDispatchStrategy<E: Event> {
    fn dispatch(&self, event: &E) -> E::Output;
}
