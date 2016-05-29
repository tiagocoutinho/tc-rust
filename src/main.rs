extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn hello_world() {
    let name = "World";
    println!("Hello, {}!", name);
}

/// Generates a secret number between [1,100]
///
/// Example:
///
/// ```
/// let secret = generate_secret_number();
/// ```
fn generate_secret_number() -> u8 {
    rand::thread_rng().gen_range(1, 101)
}

/// Asks for a number in stdin.
///
/// Example:
/// ```
/// let number = ask_number();
/// ```
fn ask_number() -> u8 {
    println!("Please guess a number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u8 = guess.trim().parse()
        .expect("Please type a number!");
    guess
}

fn compare_numbers(guess: u8, secret_number: u8) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            return true
        }
    }
    false
}

fn play_guess_number() {
    let secret_number = generate_secret_number();

    loop {
        let guess = ask_number();
        if compare_numbers(guess, secret_number) {
            break;
        }
    }
}

fn main() {
    hello_world();
    println!("");

    let play_game: fn() = play_guess_number;

    play_game();
}
