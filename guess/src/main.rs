use std::io;
use std::cmp::Ordering;
use rand::distributions::{Distribution, Uniform};


fn main() {
    println!("Guess the number!");

    let secret_number = Uniform::new_inclusive(1, 100)
        .sample(&mut rand::thread_rng());

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break
            }
        }
    }
}
