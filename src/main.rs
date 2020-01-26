use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::io::{self, BufRead};


fn is_prime(&x: &u128) -> bool {
    if x < 2 {
        false
    } else {
        ! (2..x).any(|i| {
            x % i == 0
        })
    }
}

pub fn expensive_computation(x: u32) -> u128 {
    (0..=x as u128).filter(is_prime).product()
}


pub fn main() {
    let (tx_in, rx_in) = mpsc::channel();
    let (tx_out, rx_out) = mpsc::channel();

    let _ = thread::spawn(move || {
        rx_in.iter()
            .map(expensive_computation)
            .try_for_each(move |x| {
                tx_out.send(x)
            })
    });

    let _stdin = io::stdin();
    let mut stdin = _stdin.lock();
    let mut buffer = String::with_capacity(1024);
    let mut read = || {
        stdin.read_line(&mut buffer).ok().and_then(|_| {
            let x = buffer.trim_end().parse();
            buffer.clear();
            x.ok()
        })
    };

    while read().and_then(|x| tx_in.send(x).ok()).is_some() {
        loop {
            if let Ok(ans) = rx_out.try_recv() {
                println!("{}", ans);
                break
            } else {
                println!("waiting...");
                thread::sleep(Duration::from_nanos(250))
            }
        }
    }
}
