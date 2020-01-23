#![feature(fn_traits)]
#![feature(unboxed_closures)]
use std::cell::Cell;


pub struct Cacher<A, B, F>{
    calculation: F,
    value: Cell<Option<(A, B)>>
}

impl<A: PartialEq + Copy, B: Copy, F> Cacher<A, B, F> {
    fn get(&self, key: A) -> Option<B> {
        self.value.get()
            .and_then(|(k, v)| {
                if k == key {
                    Some(v)
                } else {
                    None
                }
            })
    }

    fn set(&self, key: A, value: B) {
        self.value.set(Some((key, value)))
    }
}

impl<A: PartialEq + Copy, B: Copy, F: Fn(A) -> B> Cacher<A, B, F> {
    pub fn cache(&self, arg: A) {
        if self.get(arg).is_none() {
            self.set(arg, (self.calculation)(arg))
        }
    }
}

impl<A, B, F> From<F> for Cacher<A, B, F> {
    fn from(calculation: F) -> Self {
        Cacher {
            calculation,
            value: Cell::new(None),
        }
    }
}

impl<A: PartialEq + Copy, B: Copy, F: Fn(A) -> B> Fn<(A,)> for Cacher<A, B, F>{
    extern "rust-call" fn call(&self, (arg,): (A,)) -> B {
        self.get(arg).unwrap_or_else(|| {
            let x = (self.calculation)(arg);
            self.set(arg, x);
            x
        })
    }
}

impl<A: PartialEq + Copy, B: Copy, F: FnMut(A) -> B> FnMut<(A,)> for Cacher<A, B, F>{
    extern "rust-call" fn call_mut(&mut self, (arg,): (A,)) -> B {
        self.get(arg).unwrap_or_else(|| {
            let x = (self.calculation)(arg);
            self.set(arg, x);
            x
        })
    }
}

impl<A: PartialEq + Copy, B: Copy, F: FnOnce(A) -> B> FnOnce<(A,)> for Cacher<A, B, F>{
    type Output = B;

    extern "rust-call" fn call_once(self, (arg,): (A,)) -> B {
        self.get(arg).unwrap_or_else(|| {
            (self.calculation)(arg)
        })
    }
}
