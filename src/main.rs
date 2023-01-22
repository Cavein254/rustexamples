use std::io;

fn main() {
    let mut guess = String::new();
    println!("Guess the secret number");
    io::stdin().read_line(&mut guess).expect("Wrong number");
    println!("Your guess is {}", guess);
}
