use super::event::Event;

pub trait EventHandler<E: Event> {
    fn handle(&self, ctx: &mut E::Context, event: &E);
}
