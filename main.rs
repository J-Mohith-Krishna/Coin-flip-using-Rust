use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the Coin Flip Game!");
    println!("Guess the outcome of the coin flip (heads/tails):");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read input");
    let guess = guess.trim().to_lowercase();
    let result = if rand::thread_rng().gen_bool(0.5) { "heads" } else { "tails" };
    println!("The coin landed on: {}", result);
    
    if guess == result {
        println!("Congratulations! You guessed correctly.");
    } else {
        println!("Sorry, you guessed incorrectly.");
    }
}
