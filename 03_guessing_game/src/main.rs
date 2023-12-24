use std::io;
use std::cmp::Ordering;
use rand::Rng;

const READ_ERROR : &str = "Failed to read line";

fn main() {
    loop {
        game();

        println!("Would you like to replay ?");
        println!("0 : yes, 1: no");

        let mut replay = String::new();
        io::stdin()
            .read_line(&mut replay)
            .expect(&READ_ERROR);

        match replay.trim().parse::<u8>() {
           Ok(r) => {
               if r == 1 {
                   break;
               }
               continue;
           },
           Err(_) => {
               println!("Please choose between (y) or (n)");
               continue;
           }
        };
    }
}

fn game() {
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    let mut chances = 0;

    println!("Guess the number");
    println!("The secret number is : {secret_number}");

    loop {
        chances += 1;
        println!("Please input your guess");
        println!("Chance nº{chances}/7");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect(&READ_ERROR);

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!("You win !");
                break;
            }
        }

        if chances >= 7 {
            println!("Too much chances used !");
            println!("You loss.");
            break;
        }
    }
}
