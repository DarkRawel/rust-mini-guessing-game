use std::io;
use rand::Rng;

fn main() {
    'game: loop {
        println!("\nGuess the number!");
        let secret: i32 = rand::thread_rng().gen_range(1..=100);
        let mut i = 1;
        let mut won = false;

        while i <= 5 {
            println!("Please input your guess (1 to 100). You have {} attempts left. Type 'q' to quit.", 6 - i);

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read the line");

            let guess = guess.trim();

            if guess.eq_ignore_ascii_case("q") {
                println!("Quitting game. Goodbye!");
                break 'game;
            }

            let guess: i32 = match guess.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            };

            if guess == secret {
                println!("You guessed correctly! The secret number is {}!", secret);
                won = true;
                break;
            } else if  guess > secret{
                println!("Your guess of {} is **too high**.", guess);
            } else {
                println!("Your guess of {} is **too low**.", guess);
            }

            i += 1;
        }

        if !won {
            println!("You're out of attempts. The correct number was: {}.", secret);
        }

        println!("Restarting game...");
    }
}
