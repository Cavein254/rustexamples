use rand::Rng;
use std::io;

fn main() {
    println!("A guessing game with a loop");
    println!("Enter a number");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("You did not enter a number");

    let guess: u32 = guess.trim().parse().expect("Expected an integer");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    while secret_number != guess {
        println!("Your guess is {} answer is {}", secret_number, guess);
    }
}
