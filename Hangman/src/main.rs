use rand::seq::SliceRandom; // Use to pick element randomly from words array 

// import the display functions from display.rs
use crate::display::{display_hangman, display_word};
// Declare display modules
mod display; 

fn main() {
    let word_arr = vec![
        "apple", "beach", "smile", "shine", "blend", "coast", "brief", "flown", "dream", "swift", "charm", 
        "lemon", "porch", "scale", "quiet", "bloom", "trace", "maple", "pilot", "craft", "stone", "grape",
        "argue", "elbow", "focus", "panel", "petal", "claim", "vapor", "globe", "orbit", "spice", "burst"
    ];
    // Randomly select word from list above
    let word = word_arr.choose(&mut rand::thread_rng()).unwrap();
    // This is where we will store the guessed letters
    let mut guessed_letters = vec![];
    let mut wrong_guess_counter = 0; 
    let max_wrong_guesses=6;

    // This is where the game starts
    loop {
        display_hangman(wrong_guess_counter);
        display_word(word, &guessed_letters);
        // I also display the previously guessed letters (display until an error AKA missing char)
        println!("\nGuessed letters: {:?}", guessed_letters);
        
        println!("Enter a letter: ");
        let mut guess = String::new();
        // Here we read the user input
        std::io::stdin().read_line(&mut guess).expect("Failed to read input");
        let guess = guess.trim().to_lowercase();

        // Make sure its a single letter
        if guess.len() != 1 || ! guess.chars().all(char::is_alphabetic) {
            println!("Please enter a single letter");
            continue;
        }

        // Since we know it is a letter of length 1, we know what we got
        let guess_char = guess.chars().next().unwrap();
        
        // Now we see if the guess_char was already guessed
        if guessed_letters.contains(&guess_char) {
            println!("You already guess '{}', Try again", guess_char);
            continue;
        } else {
            guessed_letters.push(guess_char);
        }
        
        // Now we check if it is in the 
        if !word.contains(guess_char) {
            wrong_guess_counter += 1;
        }
        // Check if we exceeded max number of guesses (AKA Game Over :( .. )
        if wrong_guess_counter >= max_wrong_guesses {
            display_hangman(wrong_guess_counter);
            println!("You ran out of guesses, Game Over :(");
            println!("The answer was: {}", word);
            break;
        }
        // Check if you won
        if word.chars().all(|c|guessed_letters.contains(&c) || ! c.is_alphabetic()) {
            display_hangman(wrong_guess_counter);
            display_word(word, &guessed_letters);
            println!("\nYou Won! The word was '{}'.", word);
            break;
        }
    }
}
