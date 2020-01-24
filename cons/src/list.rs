use std::ops::Try;
use std::option::NoneError;
use std::iter::FromIterator;

pub use List::{Cons, Nil};
use super::iter::{Iter, IterMut};


type Next<T> = Box<List<T>>;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum List<T> {
    Nil,
    Cons(T, Next<T>),
}

impl<T> List<T> {
    #[inline]
    pub fn split(&mut self) -> Self {
        if let Cons(_, next) = self {
            std::mem::replace(next.as_mut(), Nil)
        } else {
            Nil
        }
    }

    #[inline]
    pub fn value(self) -> Option<T> {
        Some(self?.0)
    }

    #[inline]
    fn tail_(mut list: &mut Self) -> &mut Self {
        loop {
            match list {
                Cons(_, next) => list = next,
                Nil => break list
            }
        }
    }

    #[inline]
    pub fn tail(&mut self) -> &mut Self {
        Self::tail_(self)
    }
}

impl<T> List<T> {
    #[inline]
    pub fn new(value: T, next: Self) -> Self {
        Cons(value, Box::new(next))
    }

    #[inline]
    pub fn leaf(value: T) -> Self {
        Cons(value, Default::default())
    }

    #[inline]
    pub fn is_nil(&self) -> bool {
        match self {
            Nil => true,
            _ => false,
        }
    }

    #[inline]
    pub fn disassemble(self) -> Option<(T, Next<T>)> {
        match self {
            Nil => None,
            Cons(value, next) => Some((value, next))
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { list: self }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { list: self }
    }
}

impl<T> Iterator for List<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let mut next = self.split();
        std::mem::swap(self, &mut next);
        next.value()
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        let mut tail = self.tail();

        for value in iter {
            *tail = List::leaf(value);
            match tail {
                Cons(_, next) => tail = next,
                _ => unsafe { std::hint::unreachable_unchecked() }
            }
        }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut this = Nil;
        let mut tail = &mut this;

        for value in iter {
            *tail = List::leaf(value);
            match tail {
                Cons(_, next) => tail = next,
                _ => unsafe { std::hint::unreachable_unchecked() }
            }
        }
        this
    }
}

impl<T> Try for List<T> {
    type Ok = (T, Next<T>);
    type Error = NoneError;

    #[inline]
    fn into_result(self) -> Result<(T, Next<T>), NoneError> {
        self.disassemble().into_result()
    }

    #[inline]
    fn from_ok((value, next): (T, Next<T>)) -> Self {
        Cons(value, next)
    }

    #[inline]
    fn from_error(_: NoneError) -> Self {
        Nil
    }
}

impl<T> From<Option<T>> for List<T> {
    fn from(value: Option<T>) -> Self {
        List::leaf(value?)
    }
}

impl<T> Into<Option<(T, Next<T>)>> for List<T> {
    #[inline]
    fn into(self) -> Option<(T, Next<T>)> {
        self.disassemble()
    }
}

impl<T> Default for List<T> {
    #[inline]
    fn default() -> Self {
        Nil
    }
}

