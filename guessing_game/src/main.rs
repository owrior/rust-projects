use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game!");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Congratulations!");
                break;
            }
        }
    }
}
