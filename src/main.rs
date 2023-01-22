use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guessing Game");
    println!("Guess a number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("You did not input anything");
    println!("Your guess is {}", guess);

    let guess: u32 = guess.trim().parse().expect("not a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your guess is too low"),
        Ordering::Equal => println!("Your guess is correct!!"),
        Ordering::Greater => println!("Your guess is higher"),
    }
}
