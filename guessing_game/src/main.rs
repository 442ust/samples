use std::io;

fn main() {
    println!("Guess the number");

    println!("Input your guess: ");

    let mut guess_input = String::new();

    io::stdin()
        .read_line(&mut guess_input)
        .expect("Error when reading line");

    println!("You guessed: {}", guess_input);
}
