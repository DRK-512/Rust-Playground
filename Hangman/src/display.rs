// Remember, by default functions are private, pub -> public

pub fn display_hangman(wrong_guesses: usize){
    let stages = [
        r#"
        ----------
        |        |
                 |
                 |
                 |
                 |
        =========="#,
        r#"
        ----------
        |        |
        0        |
                 |
                 |
                 |
        =========="#,
        r#"
        ----------
        |        |
        0        |
        |        |
                 |
                 |
        =========="#,
        r#"
        ----------
        |        |
        0        |
       /|        |
                 |
                 |
        =========="#,
        r#"
        ----------
        |        |
        0        |
       /|\       |
                 |
                 |
        =========="#,
        r#"
        ----------
        |        |
        0        |
       /|\       |
       /         |
                 |
        =========="#,
        r#"
        ----------
        |        |
        0        |
       /|\       |
       / \       |
                 |
        =========="#,
    ];
    println!("{}", stages[wrong_guesses]);
}

// Displays _'s and letters that have been guessed
pub fn display_word(word: &str, guessed_letters: &[char]) {
    for c in word.chars() {
        if guessed_letters.contains(&c) {
            print!("{}", c);
        } else {
            print!("_");
        }
    }
    // Need new line after we create the built word
    println!();
}
