use crate::Handle;

pub trait System: Sized {
    fn init() -> Handle<Self>;
    fn run(&self, f: &dyn Fn(&Self)) {
        f(self);
    }
}
