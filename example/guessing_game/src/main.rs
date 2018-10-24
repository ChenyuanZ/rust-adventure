extern crate rand;  // add "rand" crate as an external dependency

use std::io;  // input/output library
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");  // println!: a macro that prints a string to the stdout

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();  // create mutable var using associated function (aka, static method)

        // The "&" indicates that this argument is a reference. References are immutable by default.
        // Hence, you need to write "&mut guess" rather than "&guess" to make it mutable.
        io::stdin().read_line(&mut guess)
            .expect("Fail to read line");

        // The colon (:) after guess tells Rust we’ll annotate the variable’s type.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // The underscore, _, is a catchall value
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
