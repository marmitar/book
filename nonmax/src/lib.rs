#![feature(const_fn)]
#![feature(const_if_match)]

use std::num::NonZeroUsize;
use std::cmp::Ordering;
use std::usize::MAX;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NonMaxUsize(NonZeroUsize);

impl NonMaxUsize {
    #[inline]
    pub const unsafe fn new_unchecked(n: usize) -> Self {
        Self(NonZeroUsize::new_unchecked(! n))
    }

    #[inline]
    pub const fn new(n: usize) -> Option<Self> {
        if n == MAX {
            None
        } else {
            Some(unsafe {
                Self::new_unchecked(n)
            })
        }
    }

    #[inline]
    pub const fn get(self) -> usize {
        ! self.0.get()
    }
}

impl Default for NonMaxUsize {
    #[inline]
    fn default() -> Self {
        unsafe { Self::new_unchecked(Default::default()) }
    }
}

impl PartialOrd for NonMaxUsize {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NonMaxUsize {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(&other.get())
    }
}

impl From<NonMaxUsize> for usize {
    #[inline]
    fn from(value: NonMaxUsize) -> usize {
        value.get()
    }
}


macro_rules! impl_from {
    (num ($( $Num: ty ),+ ) for $Ty: ident) => {
        $(
            impl From<$Num> for $Ty {
                #[inline]
                fn from(num: $Num) -> $Ty {
                    unsafe { $Ty::new_unchecked(num as usize) }
                }
            }
        )+
    };
    (nonzero ($( $Num: ident ),+ ) for $Ty: ident) => {
        $(
            impl From<std::num::$Num> for $Ty {
                #[inline]
                fn from(num: std::num::$Num) -> $Ty {
                    unsafe { $Ty::new_unchecked(num.get() as usize) }
                }
            }
        )+
    };
}

macro_rules! impl_fmt {
    (($( $Trait: ident ),+ ) for $Ty: ident ) => {
        $(
            impl std::fmt::$Trait for $Ty {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.get().fmt(f)
                }
            }
        )+
    }
}

#[cfg(target_pointer_width = "16")]
impl_from! { num (u8) for NonMaxUsize }
#[cfg(target_pointer_width = "16")]
impl_from! { nonzero (NonZeroU8) for NonMaxUsize }

#[cfg(target_pointer_width = "32")]
impl_from! { num (u8, u16) for NonMaxUsize }
#[cfg(target_pointer_width = "32")]
impl_from! { nonzero (NonZeroU8, NonZeroU16) for NonMaxUsize }

#[cfg(target_pointer_width = "64")]
impl_from! { num (u8, u16, u32) for NonMaxUsize }
#[cfg(target_pointer_width = "64")]
impl_from! { nonzero (NonZeroU8, NonZeroU16, NonZeroU32) for NonMaxUsize }

impl_fmt! { (Debug, Display, Binary, Octal, LowerHex, UpperHex) for NonMaxUsize }


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn thin_option() {
        use std::mem::size_of;

        assert_eq!(size_of::<Option<NonMaxUsize>>(), size_of::<usize>())
    }
}
