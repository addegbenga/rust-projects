// // ----> Example of guessing game using match and OrderType <------
// use rand::prelude::*;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     let random_number: u32 = rand::rng().random_range(1..=100);
//     let mut number_of_guesses: u32 = 0;
//     println!("Random number , {random_number}");
//     loop {
//         let mut user_input = String::new();
//         println!("Enter a number");
//         io::stdin()
//             .read_line(&mut user_input)
//             .expect("Failed to read line");

//         let user_input: u32 = match user_input.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Please enter a valid number!");
//                 continue; // Ask again
//             }
//         };
//         number_of_guesses = number_of_guesses + 1;

//         match user_input.cmp(&random_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win! With {} Number of attempts", number_of_guesses);
//                 break;
//             }
//         }
//     }
// }

// ----> Example of guessing game using basic if statements <------
use rand::prelude::*;
use std::io;

fn main() {
    let random_number: u32 = rand::rng().random_range(1..=100);
    let mut number_of_guesses: u32 = 0;
    println!("Random number , {random_number}");
    loop {
        let mut user_input = String::new();
        println!("Enter a number");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue; // Ask again
            }
        };
        number_of_guesses = number_of_guesses + 1;
        // Only using independent ifs
        if user_input < random_number {
            println!("Too low!");
        }
        if user_input > random_number {
            println!("Too high!");
        }
        if user_input == random_number {
            println!("You guessed it! The number was {}.", random_number);
            break; // Only break when correct
        }
        if number_of_guesses == 3 {
            break;
        }
    }
    println!("Game over. Thanks for playing! Number of guesses = {number_of_guesses}");
}
