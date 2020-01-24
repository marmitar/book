use std::ops::{Deref, DerefMut};
use std::borrow::{Borrow, BorrowMut};
use std::hash::{Hash, Hasher};
use std::fmt::{self, Pointer, Formatter, Display};


extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *const u8);
}

#[derive(Debug)]
pub struct Heaped<T> {
    ptr: *mut T
}

impl<T> Heaped<T> {
    /// # Safety
    #[inline]
    pub unsafe fn uninit() -> Self {
        Self {
            ptr: malloc(std::mem::size_of::<T>()) as *mut T
        }
    }

    #[inline]
    pub fn new(value: T) -> Self {
        let mut this = unsafe {
            Self::uninit()
        };
        *this.as_mut() = value;
        this
    }

    #[inline]
    pub fn inner(self) -> T {
        unsafe {
            std::ptr::read_unaligned(self.ptr)
        }
    }
}

impl<T> From<T> for Heaped<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Default> Default for Heaped<T> {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: Hash> Hash for Heaped<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<T: Clone> Clone for Heaped<T> {
    #[inline]
    fn clone(&self) -> Self {
        Heaped::new(self.as_ref().clone())
    }
}

impl<T> AsRef<T> for Heaped<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> AsMut<T> for Heaped<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Deref for Heaped<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.as_ref()
    }
}

impl<T> DerefMut for Heaped<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T> Borrow<T> for Heaped<T> {
    #[inline]
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T> BorrowMut<T> for Heaped<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T> Pointer for Heaped<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.ptr.fmt(f)
    }
}

impl<T: Display> Display for Heaped<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T> Drop for Heaped<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let value = std::ptr::read_unaligned(self.ptr);
            free(self.ptr as *const u8);
            std::mem::drop(value)
        }
    }
}
