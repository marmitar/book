#![feature(try_trait)]
#![feature(const_generics)]
#![allow(incomplete_features)]

pub mod list;
pub mod iter;

pub use list::List;


#[macro_export]
macro_rules! cons {
    () => (List::default());
    ($head:expr) => (List::leaf($head));
    ($head:expr, $($tail:expr),*) => (
        List::new($head, cons!($($tail),*))
    );
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_macro() {
        let mut list = cons![1, 2, 3];

        assert_eq!(list.next(), Some(1));
        assert_eq!(list.next(), Some(2));
        assert_eq!(list.next(), Some(3));
        assert_eq!(list.next(), None);
    }
}
