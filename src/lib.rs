use rand::prelude::*;
use std::error::Error;
use std::fs;
use std::io;

const MAX_MISTAKES: u32 = 5;

enum GuessReturn<'a> {
    WordRight(&'a str),
    WordWrong(&'a str),
    LetterRight(char),
    LetterWrong(char),
}

fn get_words_file_path() -> Result<String, &'static str> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Not enough arguments");
    }

    Ok(args[1].clone())
}

fn read_words_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file_content = fs::read_to_string(file_path)?;

    let words = file_content
        .split("\n")
        .filter_map(|s| {
            if s.len() > 0 {
                return Some(String::from(s.trim()).to_lowercase());
            }
            None
        })
        .collect();

    Ok(words)
}

fn choose_random_word<'a>(words: &'a Vec<String>) -> Result<&'a str, &'static str> {
    if words.len() < 1 {
        return Err("There must be at least 1 word in the file");
    }
    
    let rand_index = rand::thread_rng().gen_range(1..=words.len());
    if let Some(word) = words.get(rand_index) {
        return Ok(word);
    }

    Err("Random number generated was out of bounds")
}

fn handle_guess<'a>(guess: &'a str, secret_word: &str) -> Result<GuessReturn<'a>, &'static str> {
    let guess_chars: Vec<char> = guess.chars().collect();

    if guess_chars.len() > 1 {
        if guess == secret_word {
            return Ok(GuessReturn::WordRight(guess));
        } else {
            return Ok(GuessReturn::WordWrong(guess));
        }
    }

    if guess_chars.len() > 1 {
        return Err("The guess should be a single character at this point");
    }

    if let Some(letter) = guess_chars.get(0) {
        if secret_word.contains(guess) {
            return Ok(GuessReturn::LetterRight(*letter));
        }

        return Ok(GuessReturn::LetterWrong(*letter));
    }

    return Err("The guess should have a single character at this point");
}

fn show_word(word: &str, guessed_letters: &Vec<char>) {
    let mut output = String::from("\n");

    for c in word.chars() {
        if guessed_letters.contains(&c) {
            output.push_str(&format!("{c} "));
        } else {
            output.push_str("_ ");
        }
    }

    output.push('\n');

    for l in guessed_letters {
        output.push_str(&format!("{l} "));
    }

    println!("{output}\n");
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // gets the path to the file storing the words from the arguments environment arguments
    let words_file_path = get_words_file_path()?;

    // open a file with words separated by new lines
    let words_in_file = read_words_file(&words_file_path)?;

    // select a random word from the file
    let random_word = choose_random_word(&words_in_file)?;

    // a list of the guessed letters
    let mut guessed_letters: Vec<char> = Vec::new();

    // a count of current mistakes
    let mut mistakes: u32 = 0;

    loop {
        // shows the word and the guessed letters to the user
        show_word(&random_word, &guessed_letters);
        
        // checks wether the user still has chances left
        if mistakes >= MAX_MISTAKES {
            println!("You lost\nYou have used all your chances.");
            break;
        } else {
            println!("You have {} chances", (MAX_MISTAKES - mistakes));
        }

        let mut guess = String::new();

        // ask the user for a guess
        println!("Guess a letter or the whole word");
        io::stdin().read_line(&mut guess)?;

        // trims the input
        guess = String::from(guess.trim()).to_lowercase();

        // handle the user's guess
        if let Ok(guess_return) = handle_guess(&guess, &random_word) {
            match guess_return {
                // guessed a word and got it right
                GuessReturn::WordRight(_) => {
                    println!("You got it!");
                    break;
                }
                // guessed a word and got ir wrong
                GuessReturn::WordWrong(_) => {
                    println!("You lost :(");
                    break;
                }
                // guessed a letter and got it right
                GuessReturn::LetterRight(letter) => {
                    if guessed_letters.contains(&letter) {
                        mistakes += 1;
                    } else {
                        guessed_letters.push(letter);
                    }
                }
                // guessed a letter and got it wrong
                GuessReturn::LetterWrong(letter) => {
                    mistakes += 1;
                    if !guessed_letters.contains(&letter) {
                        guessed_letters.push(letter);
                    }
                }
            }
        }
    }

    // reveals the secret word
    println!("The word was {random_word}");

    Ok(())
}
