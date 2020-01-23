#![feature(fn_traits)]
#![feature(unboxed_closures)]
use std::cell::Cell;


pub struct Cacher<A, B, F: Fn(A) -> B>{
    calculation: F,
    value: Cell<Option<(A, B)>>
}

impl<A, B, F: Fn(A) -> B> From<F> for Cacher<A, B, F> {
    fn from(calculation: F) -> Cacher<A, B, F> {
        Cacher {
            calculation,
            value: Cell::new(None),
        }
    }
}

impl<A: PartialEq + Copy, B: Copy, F: Fn(A) -> B> Fn<(A,)> for Cacher<A, B, F>{
    extern "rust-call" fn call(&self, (arg,): (A,)) -> B {
        if let Some((key, val)) = self.value.get() {
            if key == arg {
                return val;
            }
        }

        let v = (self.calculation)(arg);
        self.value.set(Some((arg, v)));
        v
    }
}

impl<A: PartialEq + Copy, B: Copy, F: Fn(A) -> B> FnMut<(A,)> for Cacher<A, B, F>{
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> B {
        self.call(args)
    }
}

impl<A: PartialEq + Copy, B: Copy, F: Fn(A) -> B> FnOnce<(A,)> for Cacher<A, B, F>{
    type Output = B;

    extern "rust-call" fn call_once(self, args: (A,)) -> B {
        self.call(args)
    }
}
