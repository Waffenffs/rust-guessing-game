use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut player_hp = 5;
    let secret_word = rand::thread_rng().gen_range(1..=100);

    println!("Welome to the guessing game!");

    loop {
        player_hp -= 1;

        if player_hp <= 0 {
            println!("\nYou lost! Sorry! :(");
            break;
        };

        println!("\n**TOTAL REMAINING HP: {}**", player_hp);
        println!("Enter your guess: ");

        // Where the guess or answer will be stored
        let mut guess = String::new();

        // Prompt the player for a secret word and store it to guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        // Attempt to convert the type of guess' value to i32
        // If it fails, then we don't let the program crash completely
        // Instead, we let the player keep playing
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Here we compare the values of the guess and the secret word
        // If the guess is less than the secret word, we print "Secret word is less than {guess}"
        // If the guess is greater than the secret word, we print "Secret word is greater than {guess}"
        // If the guess is equal to the secret word, we print "You won! Congrats! :)" and stop the game
        match secret_word.cmp(&guess) {
            Ordering::Less => println!("Secret word is less than {}", guess),
            Ordering::Greater => println!("Secret word is greater than {}", guess),
            Ordering::Equal => {
                println!("You won! Congrats! :)");
                break;
            }
        }
    }
}
