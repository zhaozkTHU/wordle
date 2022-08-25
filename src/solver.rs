use crate::{
    all_state::ALL_STATE,
    basic_function::{judge, LetterState},
};
use rayon::prelude::*;
use tqdm::tqdm;

pub fn solver(acceptable_set: &Vec<String>, filtered_answer: &Vec<usize>) -> Vec<(String, f64)> {
    if filtered_answer.len() == 1 {
        return vec![(acceptable_set[filtered_answer[0]].clone(), 0.0)];
    }
    let mut res: Vec<(String, f64)> = vec![];
    let mut entropy: Vec<(String, f64)> = Vec::new();
    // 让可接受的单词每个都作为答案
    for i in tqdm(acceptable_set.iter()) {
        let word = i.clone();
        // println!("{}", word);
        let word_entropy = get_entropy(&word, acceptable_set, filtered_answer);
        entropy.push((word, word_entropy));
    }
    entropy.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for i in entropy.iter().enumerate() {
        if i.0 == 5 {
            break;
        }
        res.push((i.1.clone().0, i.1.clone().1));
    }
    return res;
}

pub fn filter(
    guess: &str,
    guess_state: &[LetterState; 5],
    acceptable_set: &Vec<String>,
    filtered_answer: &Vec<usize>,
) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];
    for i in filtered_answer.iter() {
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

        for (i, j) in guess_state.iter().enumerate() {
            if *j == LetterState::Y {
                let mut found = false;
                for a in word.chars().enumerate() {
                    if a.1 == guess.chars().nth(i).unwrap() && !used[a.0] && a.0 != i {
                        found = true;
                        used[a.0] = true;
                        break;
                    }
                }
                if !found {
                    qualified = false;
                    break;
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

        if qualified {
            res.push(*i);
        }
    }
    // if !filtered_answer
    //     .iter()
    //     .map(|x| acceptable_set[*x].clone())
    //     .collect::<Vec<String>>()
    //     .contains(&"amble".to_string())
    // {
    //     println!("panic {}", guess);
    // }
    return res;
}

fn get_entropy(guess: &String, acceptable_set: &Vec<String>, filtered_answer: &Vec<usize>) -> f64 {
    let mut res = [0; 243];
    for i in filtered_answer {
        let word = &acceptable_set[*i];
        let state = judge(guess, word);
        res[ALL_STATE.binary_search(&state).unwrap()] += 1;
    }
    let mut entropy: f64 = 0.0;
    for i in res.iter() {
        if *i == 0 {
            continue;
        }
        let p = *i as f64 / filtered_answer.len() as f64;
        entropy -= p * p.log2();
    }
    return entropy;
}
