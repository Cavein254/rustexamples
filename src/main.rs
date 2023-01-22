use rand::Rng;
use std::io;

fn main() {
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Enter your number");
    io::stdin().read_line(&mut guess).expect("Wrong number");
    println!("Your guess is {}", guess);
    println!("The secret Number is {}", secret_number);
}
