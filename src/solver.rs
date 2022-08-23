use std::collections::HashSet;

use crate::all_state::ALL_STATE;
use crate::basic_function::LetterState;

pub fn solver(
    known_info: &Vec<(String, [LetterState; 5])>,
    acceptable_set: &Vec<String>,
) -> Vec<String> {
    let acceptable = filter(known_info, acceptable_set);
    if acceptable.len() <= 5 {
        return acceptable;
    }
    let mut entropy: Vec<(f32, String)> = acceptable
        .iter()
        .map(|x| (get_entropy(x, acceptable_set, known_info), x.clone()))
        .collect();
    println!("{} {}", entropy[0].1, entropy[0].0);
    entropy.sort_by(|a, b| b.0.total_cmp(&a.0));
    let mut res: Vec<String> = vec![];
    for i in 0..5 {
        res.push(entropy[i].1.clone());
    }
    return res;
}

pub fn filter(
    known_info: &Vec<(String, [LetterState; 5])>,
    acceptable_set: &Vec<String>,
) -> Vec<String> {
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
    for word in acceptable_set.iter() {
        let mut matched = [false; 5];
        let mut acc = true;

        for i in green_letter.iter() {
            if word.chars().nth(i.1).unwrap() != i.0 {
                acc = false;
                break;
            }
            matched[i.1] = true;
        }
        if !acc {
            continue;
        }

        for red in red_letter.iter() {
            if word.find(red.clone()).is_some() && !matched[word.find(red.clone()).unwrap()] {
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

fn get_entropy(
    word: &str,
    acceptable_set: &Vec<String>,
    known_info: &Vec<(String, [LetterState; 5])>,
) -> f32 {
    let mut known_info = known_info.clone();
    let mut rate: [u32; 243] = [0; 243];

    // calculate rate of every state
    for answer in acceptable_set {
        let state = crate::basic_function::judge(word, answer);
        rate[ALL_STATE.binary_search(&state).unwrap() as usize] += 1;
    }

    let mut res: f32 = 0.0;

    for i in 0..243 {
        let rate = rate[i] as f32 / 243.0;
        if rate == 0.0 {
            continue;
        }

        // calculate the rate of words can be filtered by the word
        known_info.push((word.to_string(), ALL_STATE[i]));
        let filtered = filter(&known_info, acceptable_set).len();
        let p = filtered as f32 / acceptable_set.len() as f32;

        res += -p.ln() * rate;
    }

    return res;
}
