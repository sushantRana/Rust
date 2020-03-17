use std::io;
use std::cmp::Ordering;

use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 21);
    println!("Guess the number!");
    loop {
        println!("Please enter your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");// {
            let guess:u32 = guess.trim().parse().expect("Please type a number!"); 
            println!("you choosed {}", guess);
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                },
            }
        }
    }
