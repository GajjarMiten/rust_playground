use core::num;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failt to read line");
    println!("You guessed: {guess}");

    let guess = guess.trim().parse().expect("not a number");

    match guess.cmp(&number) {
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("Perfect match!"),
    }
}
