use core::cell::UnsafeCell;

#[derive(Clone)]
pub struct Handle<S: ?Sized>(*mut S);

impl<S: ?Sized> core::ops::Deref for Handle<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<S: ?Sized> core::ops::DerefMut for Handle<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

pub struct Node<S: ?Sized> {
    sys: UnsafeCell<S>,
}

impl<S> Node<S> {
    pub const fn new(sys: S) -> Self {
        Self {
            sys: UnsafeCell::new(sys),
        }
    }

    pub fn handle(&self) -> Handle<S> {
        Handle(self.sys.get())
    }
}
