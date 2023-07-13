use std::collections::HashMap;
use std::io;

// A list of dictionary WORDS for the game of hangman.
static WORDS: &[&str] = &[
    "hello",
    "world",
    "rust",
    "programming",
    "language",
    "computer",
    "science",
];

fn choose_word() -> &'static str {
    // Roll a random number between 0 and the length of the WORDS vector.
    let random_number = rand::random::<usize>() % WORDS.len();

    // Return the word at the random index.
    WORDS[random_number]
}

fn game_loop(
    letters_guessed: &mut HashMap<char, bool>,
    secret_word_letters: &[char],
    allowed_guesses: i8,
) {
    // Display the secret word with underscores for unguessed letters.
    println!("The secret word is:");
    let mut won = true;
    let mut incorrect_guesses = 0;
    // count the number of incorrect guesses
    for letter in letters_guessed.keys() {
        if !secret_word_letters.contains(letter) {
            incorrect_guesses += 1;
        }
    }
    // print out remaining guesses
    println!(
        "You have {} guesses left.",
        allowed_guesses - incorrect_guesses
    );

    for letter in secret_word_letters {
        if letters_guessed.get(letter).is_some() {
            print!("{} ", letter);
        } else {
            won = false;
            print!("_ ");
        }
    }
    if won {
        println!("You won!");
        return;
    } else if (allowed_guesses - incorrect_guesses) == 0 {
        println!("You lost!");
        return;
    }

    // Ask the user to guess a letter.
    println!("\nGuess a letter:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You guessed: {input}");
        }
        Err(error) => println!("error: {error}"),
    }

    // Add the letter to the letters_guessed HashMap.
    letters_guessed.insert(input.chars().next().unwrap(), true);

    // Display all guessed letters so far.
    println!("Guessed letters:");
    for (letter, _) in &mut *letters_guessed {
        print!("{} ", letter);
    }
    game_loop(letters_guessed, secret_word_letters, allowed_guesses);
}

fn main() {
    let secret_word = choose_word();
    // Transform the secret word into a vector of characters.
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    let mut letters_guessed = HashMap::new();
    game_loop(&mut letters_guessed, &secret_word_chars, 6);
}
