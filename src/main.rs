use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let mut player_lives = 4;
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {}", secret_number);

    loop {
        player_lives -= 1;

        if player_lives == 0 {
            println!("You lost!");
            break;
        }

        println!("Your remaining lives: {}", player_lives);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read value");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}", guess);

        match secret_number.cmp(&guess) {
            Ordering::Less => println!("The secret number is smaller!"),
            Ordering::Greater => println!("The secret number is greater!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
