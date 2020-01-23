#![feature(fn_traits)]
#![feature(unboxed_closures)]
use std::thread;
use std::time::Duration;
use std::cell::Cell;


struct Cacher<A, B, F: Fn(A) -> B>{
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


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = Cacher::from(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result(intensity)
            );
        }
    }
}
