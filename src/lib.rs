use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;


#[deriving(Clone)]
pub struct CowRc<T> {
    v: Rc<T>
}

#[deriving(Clone)]
pub struct CowArc<T> {
    v: Arc<T>
}

impl <T> Deref<T> for CowRc<T> {
    fn deref(&self) -> &T {
        self.v.deref()
    }
}

impl <T: Clone> DerefMut<T> for CowRc<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.v.make_unique()
    }
}

impl <T> Deref<T> for CowArc<T> {
    fn deref(&self) -> &T {
        self.v.deref()
    }
}

impl <T: Clone + Sync + Send> DerefMut<T> for CowArc<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.v.make_unique()
    }
}
