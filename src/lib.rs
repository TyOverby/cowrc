#![feature(alloc)]
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;


#[derive(Clone, Hash, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CowRc<T> {
    v: Rc<T>
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CowArc<T> {
    v: Arc<T>
}

impl <T: Clone> CowRc<T> {
    pub fn new(t: T) -> CowRc<T> {
        CowRc {
            v: Rc::new(t)
        }
    }
}

impl <T: Clone + Sync + Send> CowArc<T> {
    pub fn new(t: T) -> CowArc<T> {
        CowArc {
            v: Arc::new(t)
        }
    }
}

impl <T> Deref for CowRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.v.deref()
    }
}

impl <T: Clone> DerefMut for CowRc<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.v.make_unique()
    }
}

impl <T: Clone + Sync + Send> Deref for CowArc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.v.deref()
    }
}

impl <T: Clone + Sync + Send> DerefMut for CowArc<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.v.make_unique()
    }
}

#[test]
fn test_cow() {
    let st = CowRc::new("hello".to_string());
    let mut nd = st.clone();
    nd.push_str(" world");
    nd.push_str("!");

    assert!(*st == "hello");
    assert!(*nd == "hello world!");


    let v = CowRc::new(vec![1u32,2,3,4]);
    let mut v2 = v.clone();
    v2.push(5);
    let mut v3 = v.clone();
    v3.push(0);
    let mut v4 = v.clone();
    v4.push(5);
    v4.push(6);
    v4.push(7);
    v4.push(8);

    assert!(*v == vec![1,2,3,4]);
    assert!(*v2 == vec![1,2,3,4,5]);
    assert!(*v3 == vec![1,2,3,4,0]);
    assert!(*v4 == vec![1,2,3,4,5,6,7,8]);
}
