use crate::builtin_words::*;
use crate::Opt;
use rand::Rng;
use std::io::stdin;

/// G > Y > R > X
#[derive(PartialEq, Copy, Clone, Eq, PartialOrd, Ord, Debug)]
enum LetterState {
    X,
    R,
    Y,
    G,
}

/// Return the words state
fn judge(words: &str, answer: &str) -> Vec<LetterState> {
    use LetterState::*;
    let mut words_state = vec![R, R, R, R, R];
    let mut answer_state = vec![R, R, R, R, R];
    let words_vec: Vec<char> = words.chars().collect();
    let answer_vec: Vec<char> = answer.chars().collect();
    for i in 0..5 {
        if words_vec[i] == answer_vec[i] {
            words_state[i] = G;
            answer_state[i] = G;
        }
    }
    for i in 0..5 {
        if words_state[i] != G {
            for j in 0..5 {
                if (words_vec[i] == answer_vec[j]) && (answer_state[j] == R) {
                    words_state[i] = Y;
                    answer_state[j] = Y;
                }
            }
        }
    }

    return words_state;
}

/// Interactivate_mode || Not perfect now
pub fn interactivate_mode() {
    let answer_word: String = FINAL[rand::thread_rng().gen_range(0..FINAL.len())].to_string();
    // println!("{}", &answer_word);
    let mut win = false;
    for i in 0..6 {
        let mut guess_words: String;
        let mut correct = 0;
        loop {
            guess_words = text_io::read!();
            if guess_words.len() < 5 {
                print!("{}", console::style("Not enough letters").bold().red());
                continue;
            }
            if guess_words.len() > 5 {
                print!("{}", console::style("Too many letters.").bold().red());
            }
            if !ACCEPTABLE.contains(&guess_words.as_str()) {
                println!("{}", console::style("Not in word list").bold().red());
                continue;
            }
            break;
        }
        for i in 0..guess_words.len() {
            let answer_vec: Vec<char> = answer_word.clone().chars().collect();
            let guess_vec: Vec<char> = guess_words.clone().chars().collect();
            if answer_vec.contains(&guess_vec[i]) {
                if answer_vec[i] == guess_vec[i] {
                    print!("{}", console::style(&guess_vec[i]).green());
                    correct += 1;
                } else {
                    print!("{}", console::style(&guess_vec[i]).yellow());
                }
            } else {
                print!("{}", console::style(&guess_vec[i]).red());
            }
        }
        print!("\n");
        if correct == 5 {
            println!(
                "{}",
                console::style(format!(
                    "Congratulations, you find the answer! You have tried for {} time(s).",
                    i + 1
                ))
                .green()
                .bold()
            );
            win = true;
            break;
        }
    }
    if win == false {
        println!(
            "{}\n{}",
            console::style("Sorry, you lose the game.Please try again.")
                .red()
                .bold(),
            console::style(format!("The answer is {}", &answer_word))
                .red()
                .bold()
        )
    }
}

/// Test mode
pub fn test_mode(word: Option<String>) {
    let mut answer_word: String = String::new();
    if word.is_some() {
        answer_word = word.unwrap();
    } else {
        stdin().read_line(&mut answer_word).expect("");
    }
    // let answer_word: String = FINAL[rand::thread_rng().gen_range(0..FINAL.len())].to_string();
    let mut keyboard = [LetterState::X; 26];
    let mut win = false;
    let mut tries = 0;
    for _ in 0..6 {
        let mut guess = String::new();
        // Read until valid
        loop {
            guess.clear();
            stdin().read_line(&mut guess).expect("");
            guess = guess.trim().to_string();
            // unqualified length
            if guess.trim().chars().count() != 5 {
                println!("INVALID");
                continue;
            }
            // should be all letters
            if guess.trim().chars().any(|x| !x.is_ascii_alphabetic()) {
                println!("INVALID");
                continue;
            }
            // not in word list
            if !ACCEPTABLE.contains(&guess.as_str()) {
                println!("INVALID");
                continue;
            }
            break;
        }

        tries += 1;
        let state = judge(guess.trim(), &answer_word);
        for i in 0..5 {
            print!("{:?}", state[i]);
            // Update the keyboard state
            // guess.chars().nth(i).unwrap().to_ascii_lowercase() as usize - 'a' as usize   the index of a ascii character
            if keyboard[guess.chars().nth(i).unwrap().to_ascii_lowercase() as usize - 'a' as usize]
                < state[i]
            {
                keyboard
                    [guess.chars().nth(i).unwrap().to_ascii_lowercase() as usize - 'a' as usize] =
                    state[i].clone();
            }
        }
        print!(" ");
        for i in keyboard.iter() {
            print!("{:?}", i);
        }
        print!("\n");
        if state.iter().all(|x| *x == LetterState::G) {
            win = true;
            break;
        }
    }
    if win == true {
        println!("CORRECT {}", tries);
    } else {
        println!("FAILED {}", answer_word.to_ascii_uppercase());
    }
}
