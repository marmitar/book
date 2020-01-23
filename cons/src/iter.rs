use std::iter::FusedIterator;

use super::List::{self, Cons, Nil};


pub struct Iter<'a, T> {
    pub (super) list: &'a List<T>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.list {
            Nil => None,
            Cons(value, next) => {
                self.list = next;
                Some(value)
            }
        }
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> { }


pub struct IterMut<'a, T> {
    pub (super) list: &'a mut List<T>
}

unsafe fn map_lifetime<'a, T>(value: &mut T) -> &'a mut T {
    &mut *(value as *mut T)
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        match self.list {
            Nil => None,
            Cons(value, next) => unsafe {
                self.list = map_lifetime(next);
                Some(map_lifetime(value))
            }
        }
    }
}

impl<'a, T> FusedIterator for IterMut<'a, T> { }
