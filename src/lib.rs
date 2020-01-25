use std::ops::{Deref, DerefMut};
use std::borrow::{Borrow, BorrowMut};
use std::hash::{Hash, Hasher};
use std::fmt::{self, Pointer, Formatter, Display};

use alloc::Ptr;


mod alloc {
    extern "C" {
        fn malloc(size: usize) -> *mut u8;

        fn free(ptr: *const u8);
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Ptr<T>(*mut T);

    impl<T> Ptr<T> {
        pub const SIZE: usize = std::mem::size_of::<T>();

        /// # Safety
        #[inline]
        pub unsafe fn alloc() -> Self {
            Ptr(malloc(Self::SIZE) as *mut T)
        }

        /// # Safety
        #[inline]
        pub unsafe fn read(&self) -> T {
            std::ptr::read(self.0)
        }

        /// # Safety
        #[inline]
        pub unsafe fn write(&self, value: T) {
            std::ptr::write(self.0, value)
        }

        #[inline]
        pub fn inner(&self) -> *mut T {
            self.0
        }

        /// # Safety
        #[inline]
        pub unsafe fn free(&self) {
            free(self.0 as *const u8)
        }
    }

    impl<T> Drop for Ptr<T> {
        #[inline]
        fn drop(&mut self) {
            unsafe { self.free() }
        }
    }
}


#[derive(Debug)]
pub struct Heaped<T> {
    ptr: Ptr<T>
}

impl<T> Heaped<T> {
    /// # Safety
    #[inline]
    pub unsafe fn uninit() -> Self {
        Self { ptr: Ptr::alloc() }
    }

    #[inline]
    pub fn new(value: T) -> Self {
        unsafe {
            let this = Self::uninit();
            this.ptr.write(value);
            this
        }
    }

    #[inline]
    pub fn inner(self) -> T {
        unsafe {
            let value = self.ptr.read();
            self.ptr.free();
            std::mem::forget(self);
            value
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
        unsafe { &*self.ptr.inner() }
    }
}

impl<T> AsMut<T> for Heaped<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut*self.ptr.inner() }
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
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.ptr.inner().fmt(f)
    }
}

impl<T: Display> Display for Heaped<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T> Drop for Heaped<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { std::ptr::drop_in_place(self.ptr.inner()) }
    }
}
