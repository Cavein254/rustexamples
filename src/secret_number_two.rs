use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("A guessing game with a loop");
    loop {
        println!("Enter a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("You did not enter a number");

        let guess: u32 = guess.trim().parse().expect("Expected an integer");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        match guess.cmp(&secret_number) {
            Ordering::Equal => println!("You guessed right"),
            Ordering::Greater => println!("Bigger than answer"),
            Ordering::Less => {
                println!("Too small");
                break;
            }
        }
    }
}
