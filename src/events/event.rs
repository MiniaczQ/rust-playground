pub trait Event: Sized {
    type Context;
    type Output;
}
