use std::collections::HashSet;

use crate::{basic_function::LetterState, builtin_words::ACCEPTABLE};

pub fn solver(known_info: &Vec<(String, [LetterState; 5])>) -> Vec<String> {
    let mut red_letter: HashSet<char> = HashSet::new();
    let mut green_letter: HashSet<(char, usize)> = HashSet::new();
    let mut yellow_letter: HashSet<(char, usize)> = HashSet::new();
    for (word, state) in known_info.iter() {
        for i in 0..5 {
            match state[i] {
                LetterState::G => {
                    green_letter.insert((word.chars().nth(i).unwrap(), i));
                }
                LetterState::R => {
                    red_letter.insert(word.chars().nth(i).unwrap());
                }
                LetterState::Y => {
                    yellow_letter.insert((word.chars().nth(i).unwrap(), i));
                }
                _ => {}
            }
        }
    }

    let mut res: Vec<String> = vec![];
    for word in ACCEPTABLE.iter() {
        let mut acc = true;
        for red in red_letter.iter() {
            if word.contains(&red.to_string()) {
                acc = false;
                break;
            }
        }
        if !acc {
            continue;
        }
        for i in green_letter.iter() {
            if word.chars().nth(i.1).unwrap() != i.0 {
                acc = false;
                break;
            }
        }
        if !acc {
            continue;
        }
        for i in yellow_letter.iter() {
            if !word.contains(i.0) {
                acc = false;
                break;
            }
            if word.chars().nth(i.1).unwrap() == i.0 {
                acc = false;
                break;
            }
        }
        if acc {
            res.push(word.to_string());
        }
    }
    return res;
}
