use crate::builtin_words::*;
use crate::Opt;
use rand::Rng;
use std::collections::BTreeMap;
use std::io::stdin;

/// G > Y > R > X
#[derive(PartialEq, Copy, Clone, Eq, PartialOrd, Ord, Debug)]
enum LetterState {
    X,
    R,
    Y,
    G,
}

pub fn test_mode(opt: &Opt) {
    let mut total_win: u32 = 0;
    let mut total_lose: u32 = 0;
    let mut total_tries: u32 = 0;
    let mut used_answer: Vec<String> = Vec::new();
    let mut words_frequency: BTreeMap<String, u32> = BTreeMap::new();
    loop {
        let answer_word: String = get_answer_word(opt, &mut used_answer);

        let mut keyboard = [LetterState::X; 26];
        let mut win = false;
        let mut tries = 0;
        let mut last_word: Option<String> = None;

        for _ in 0..6 {
            let guess = input_guess(&opt, &last_word, &answer_word, &mut words_frequency);
            last_word = Some(guess.clone());

            tries += 1;
            let state = judge(&guess.trim(), &answer_word.trim());
            for i in 0..5 {
                print!("{:?}", state[i]);
                // Update the keyboard state
                // guess.chars().nth(i).unwrap().to_ascii_lowercase() as usize - 'a' as usize   the index of a ascii character
                if keyboard[guess.chars().nth(i).unwrap() as usize - 'a' as usize] < state[i] {
                    keyboard[guess.chars().nth(i).unwrap() as usize - 'a' as usize] =
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
            total_win += 1;
            total_tries += tries;
        } else {
            println!("FAILED {}", answer_word.trim().to_ascii_uppercase());
            total_lose += 1;
        }
        if opt.stats == true {
            println!(
                "{} {} {:.2}",
                total_win,
                total_lose,
                if total_win == 0 {
                    0 as f32
                } else {
                    total_tries as f32 / total_win as f32
                }
            );
            // TODO
            let mut tmp = words_frequency.clone();
            for _ in 0..5 {
                if tmp.is_empty() {
                    break;
                }
                let max = tmp.iter().rev().max_by_key(|x| x.1).unwrap();
                let a = max.0.clone();
                print!("{} {} ", max.0.to_ascii_uppercase(), max.1);
                tmp.remove(&a);
            }
            print!("\n");
        }
        if opt.word.is_none() || opt.stats {
            let mut again = String::new();
            let bytes = stdin().read_line(&mut again).unwrap();
            if again.trim() == "N" || bytes == 0 {
                break;
            }
            if again.trim() == "Y" {
                continue;
            }
            break;
        }
    } // Loop end
}

/// Return the words state
fn judge(guess: &str, answer: &str) -> Vec<LetterState> {
    use LetterState::*;
    let mut words_state = vec![R, R, R, R, R];
    let mut answer_state = vec![R, R, R, R, R];
    let words_vec: Vec<char> = guess.chars().collect();
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
                    break;
                }
            }
        }
    }

    return words_state;
}

fn get_answer_word(opt: &Opt, used_answer: &mut Vec<String>) -> String {
    let mut answer_word: String = String::new();
    if opt.random == true {
        loop {
            answer_word = FINAL[rand::thread_rng().gen_range(0..FINAL.len())].to_string();
            if !used_answer.contains(&answer_word) {
                used_answer.push(answer_word.clone());
                break;
            }
        }
    } else {
        if opt.word.is_some() {
            answer_word = opt.word.clone().unwrap();
        } else {
            stdin().read_line(&mut answer_word).expect("");
        }
    }
    return answer_word;
}

fn check_guess_in_difficult(guess: &String, last_word: &String, answer: &String) -> bool {
    let word_state = judge(guess, answer);
    let last_state = judge(last_word, answer);
    let mut used = vec![];
    // All green should be same
    for i in 0..5 {
        if last_state[i] == LetterState::G {
            if word_state[i] != LetterState::G {
                return false;
            }
            used.push(i);
        }
    }
    // Yellow letter should be used again or be green
    for i in 0..5 {
        if last_state[i] == LetterState::Y {
            let mut matched = false;
            for j in 0..5 {
                if (!used.contains(&j))
                    && word_state[j] >= last_state[i]
                    && guess.chars().nth(j) == last_word.chars().nth(i)
                {
                    used.push(j);
                    matched = true;
                    break;
                }
            }
            if matched == false {
                return false;
            }
        }
    }
    return true;
}

/// Handle the guess input
fn input_guess(
    opt: &Opt,
    last_word: &Option<String>,
    answer: &String,
    word_frequency: &mut BTreeMap<String, u32>,
) -> String {
    let mut guess = String::new();
    loop {
        guess.clear();
        stdin().read_line(&mut guess).expect("");
        guess = guess.trim().to_string().to_ascii_lowercase();
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
        // difficult mode
        if opt.difficult == true {
            if last_word.is_some() {
                if !check_guess_in_difficult(&guess, last_word.clone().as_mut().unwrap(), &answer) {
                    println!("INVALID");
                    continue;
                }
            }
        }
        break;
    }
    *word_frequency.entry(guess.clone()).or_insert(0) += 1;
    return guess;
}
