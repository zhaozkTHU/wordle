use crate::{
    all_state::ALL_STATE,
    basic_function::{judge, LetterState},
};
use tqdm::tqdm;

pub fn solver(acceptable_set: &Vec<String>, hint_acceptable: &Vec<usize>) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut entropy: Vec<(String, f64)> = Vec::new();
    // 让可接受的单词每个都作为答案
    for i in tqdm(hint_acceptable.iter()) {
        let word = acceptable_set[*i].clone();
        // println!("{}", word);
        let word_entropy = get_entropy(&word, acceptable_set, hint_acceptable);
        entropy.push((word, word_entropy));
    }
    entropy.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for i in entropy.iter().enumerate() {
        if i.0 == 5 {
            break;
        }
        res.push(i.1.clone().0);
    }
    return res;
}

pub fn filter(
    guess: &str,
    guess_state: &[LetterState; 5],
    acceptable_set: &Vec<String>,
    hint_acceptable: &Vec<usize>,
) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];
    for i in hint_acceptable.iter() {
        let word = acceptable_set[*i].as_str();
        let mut used = [false; 5];

        let mut qualified = true;
        for (i, j) in guess_state.iter().enumerate() {
            if *j == LetterState::G {
                if guess.chars().nth(i).unwrap() != word.chars().nth(i).unwrap() {
                    qualified = false;
                    break;
                } else {
                    used[i] = true;
                }
            }
        }
        if !qualified {
            continue;
        }

        for j in guess_state.iter().enumerate() {
            if *j.1 == LetterState::R {
                for (i, k) in word.chars().enumerate() {
                    if k == guess.chars().nth(j.0).unwrap() && !used[i] {
                        qualified = false;
                        break;
                    }
                }
            }
        }
        if !qualified {
            continue;
        }

        for (i, j) in guess_state.iter().enumerate() {
            if *j == LetterState::Y {
                let mut found = false;
                for a in word.chars().enumerate() {
                    if a.1 == guess.chars().nth(i).unwrap() && !used[i] {
                        found = true;
                        used[i] = true;
                        break;
                    }
                }
                if !found {
                    qualified = false;
                    break;
                }
            }
        }

        if qualified {
            res.push(*i);
        }
    }
    return res;
}

fn get_entropy(guess: &String, acceptable_set: &Vec<String>, hint_acceptable: &Vec<usize>) -> f64 {
    let mut res = [0; 243];
    for i in hint_acceptable {
        let word = &acceptable_set[*i];
        let state = judge(guess, word);
        res[ALL_STATE.binary_search(&state).unwrap()] += 1;
    }
    let mut entropy: f64 = 0.0;
    for i in res.iter() {
        if *i == 0 {
            continue;
        }
        let p = *i as f64 / hint_acceptable.len() as f64;
        entropy -= p * p.ln();
    }
    return entropy;
}
