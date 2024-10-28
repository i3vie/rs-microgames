use rand::seq::SliceRandom;
use std::{io::{stdin, stdout, Write}, iter, process::exit};
mod wordlist;

fn main() {
    let stdin = stdin();
    let word: String = String::from(wordlist::WORDLIST.choose(&mut rand::thread_rng()).unwrap().to_owned()).to_lowercase();

    let mut wrong_guesses: Vec<String> = Vec::new();
    let mut all_guesses  : Vec<String> = Vec::new();

    let mut mistakes: i8 = 0;

    let mut display: Vec<char>;

    display = iter::repeat("-")
                .take(word.len())
                .collect::<Vec<_>>()
                .join(" ")
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect();

    print!("\x1B[2J\x1B[1;1H");
    loop {
        println!("WORD:    {}", display.iter().collect::<String>());

        if !display.contains(&'-') {
            println!("YOU WIN!");
            exit(0)
        }

        println!("WRONG:   {} ({}/6)", wrong_guesses.join(", "), mistakes);
        print!("guess: ");
        stdout().flush().unwrap();

        let mut guess = String::from("");

        let _ = stdin.read_line(&mut guess);
        guess = guess[0..1].to_string().to_lowercase();

        if all_guesses.contains(&guess) {
            print!("\x1B[2J\x1B[1;1H");
            continue;
        }

        let old_string = display.clone();
        for (i, c) in word.chars().enumerate() {
            if c.to_string() == guess {
                display[i] = guess.chars().next().unwrap();
            }
        }
        if old_string == display {
            mistakes += 1;
            wrong_guesses.push(guess.clone());
        }
        all_guesses.push(guess);
        if mistakes == 6 {
            println!("You ran out of guesses!");
            println!("The word was {}", word);
            exit(1);
        }
        print!("\x1B[2J\x1B[1;1H");
    }
}
