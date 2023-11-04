use std::io;
use rand::Rng;
use std::mem::drop;

fn main() {
    let mut cont = 1;

    while cont == 1 {
        guesser();

        println!("Start a new round? Y/N");

        // Read user input
        let mut user_continue = String::new();

        io::stdin()
            .read_line(&mut user_continue)
            .expect("Failed to read line");

        if user_continue.trim().eq("N") {
            cont = 0;
        }
    }
}

/**
 * Start Guesser
 */
fn guesser() {
    println!("Guess the number between 1 and 10");

    // Generate random number
    let random_number = create_random_number();

    let user_number = get_user_input();
    
    // Check if numbers are equal
    check_numbers(random_number, user_number)
}

/**
 * Create random number
 */
fn create_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..11)
}

/**
 * Get number from user
 */
fn get_user_input() -> i32 {
    loop { 
        let mut guess = String::new();
        println!("Please input your guess:");
    
        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    

        // Check if input is numeric
        if guess.chars().next().unwrap().is_numeric() {
            return guess.trim_end().parse::<i32>().unwrap();
        }
        else {
            println!("That was not a number");
            drop(guess);
        }
    }
}

/**
 * Check if numbers are equal
 */
fn check_numbers(random: i32, user: i32) {
    if  random == user {
        println!("You guessed right, the number was: {}", random);
    } else {
        println!("You guessed wrong, the number was: {}", random);
    };
}