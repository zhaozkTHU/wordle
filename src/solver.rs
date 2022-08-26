use std::io::stdin;

use crate::{
    all_state::ALL_STATE,
    basic_function::{judge, LetterState},
    builtin_words::ACCEPTABLE,
};
use indicatif::ParallelProgressIterator;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn inter_solver() {
    let acceptable_set: Vec<String> = ACCEPTABLE
        .to_vec()
        .iter_mut()
        .map(|x| x.to_string())
        .collect();
    let mut filtered_answer: Vec<usize> = (0..acceptable_set.len()).collect();

    println!("Please input: tares\n");
    let mut guess = "tares".to_string();
    loop {
        println!("Please input the word state, like YRGYR\n");

        let mut guess_state = [LetterState::X; 5];
        let mut tmp: String = String::new();
        stdin().read_line(&mut tmp).unwrap();
        tmp = tmp.trim().chars().map(|x| x.to_ascii_uppercase()).collect();
        for i in tmp.chars().enumerate() {
            match i.1 {
                'G' => guess_state[i.0] = LetterState::G,
                'Y' => guess_state[i.0] = LetterState::Y,
                'R' => guess_state[i.0] = LetterState::R,
                _ => unreachable!(),
            }
        }

        filtered_answer = filter(&guess, &guess_state, &acceptable_set, &filtered_answer);
        guess = solver(&acceptable_set, &filtered_answer)[0].0.clone();
        if solver(&acceptable_set, &filtered_answer).len() == 1 {
            println!("The answer is {}", guess.to_ascii_uppercase());
            return;
        } else {
            println!("Please input: {}", guess.to_ascii_uppercase());
        }
    }
}

pub fn solver(acceptable_set: &Vec<String>, filtered_answer: &Vec<usize>) -> Vec<(String, f64)> {
    if filtered_answer.len() == 1 {
        return vec![(acceptable_set[filtered_answer[0]].clone(), 0.0)];
    }
    let mut res: Vec<(String, f64)> = vec![];

    let mut entropy: Vec<_> = acceptable_set
        .par_iter()
        .progress_count(acceptable_set.len() as u64)
        .map(|i| (i.clone(), get_entropy(i, acceptable_set, filtered_answer)))
        .collect();

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
        // abaca YRRRR   psalm
        for i in guess.chars().zip(word.chars()).enumerate() {
            if i.1 .0 == i.1 .1 && guess_state[i.0] != LetterState::G {
                qualified = false;
            }
        }
        if !qualified {
            continue;
        }

        for (i, j) in guess_state.iter().enumerate() {
            if *j == LetterState::Y {
                let mut found = false;
                for a in word.chars().enumerate() {
                    if a.1 == guess.chars().nth(i).unwrap() && !used[a.0] {
                        if a.0 == i {
                            break;
                        } else {
                            found = true;
                            used[a.0] = true;
                            break;
                        }
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
        let p = (*i as f64) / (filtered_answer.len() as f64);
        entropy -= p * p.log2();
    }
    return entropy;
}
