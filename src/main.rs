use std::*;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{

        println!("Please input your guess!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Guess must be a number!");
                continue;
            }
        };
        println!("You guessed: {guess}"); 
        match guess.cmp(&secret_number){
            cmp::Ordering::Less => println!("Too Small!"),
            cmp::Ordering::Greater => println!("Too Big!"),
            cmp::Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
        //println!("Secret Number: {secret_number}");
    }

}

