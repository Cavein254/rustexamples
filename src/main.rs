use std::io;
fn main() {
    println!("Guesing Game:::");
    println!("Enter the input number->");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read line");

    println!("You Guessed {}", guess);
}
