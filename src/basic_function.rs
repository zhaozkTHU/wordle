use crate::Opt;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;

/// G > Y > R > X
#[derive(PartialEq, Copy, Clone, Eq, PartialOrd, Ord, Debug)]
pub enum LetterState {
    X,
    R,
    Y,
    G,
}

pub struct KeyboardState {
    pub keyboard_state: [LetterState; 26],
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            keyboard_state: [LetterState::X; 26],
        }
    }
    pub fn update(&mut self, guess: &String, game_state: &[LetterState; 5]) {
        for i in 0..5 {
            // Update the keyboard state
            // guess.chars().nth(i).unwrap().to_ascii_lowercase() as usize - 'a' as usize   the index of a ascii character
            if self.keyboard_state[guess.chars().nth(i).unwrap() as usize - 'a' as usize]
                < game_state[i]
            {
                self.keyboard_state[guess.chars().nth(i).unwrap() as usize - 'a' as usize] =
                    game_state[i].clone();
            }
        }
    }
    pub fn to_string(&self) -> String {
        let mut res = String::new();
        for i in self.keyboard_state.iter() {
            res += &format!("{:?}", i)
        }
        return res;
    }
}

/// Data which will be loaded and saved in json file
pub struct GameData {
    total_win: u32,
    total_lose: u32,
    total_tries: u32,
    pub used_answer: Vec<String>,
    words_frequency: BTreeMap<String, u32>,
}

impl GameData {
    pub fn new() -> GameData {
        GameData {
            total_win: 0,
            total_lose: 0,
            total_tries: 0,
            used_answer: Vec::new(),
            words_frequency: BTreeMap::new(),
        }
    }
    pub fn add_win(&mut self) {
        self.total_win += 1;
    }
    pub fn add_lose(&mut self) {
        self.total_lose += 1;
    }
    pub fn insert_word_frequency(&mut self, guess: &String) {
        *self
            .words_frequency
            .entry(guess.to_string().to_ascii_lowercase())
            .or_insert(0) += 1;
    }
    pub fn add_tries(&mut self, tries: u32) {
        self.total_tries += tries;
    }
    pub fn push_used_answer(&mut self, answer: &String) {
        self.used_answer.push(answer.to_string());
    }
    pub fn get_win(&self) -> u32 {
        return self.total_win;
    }
    pub fn get_lose(&self) -> u32 {
        return self.total_lose;
    }
    pub fn get_words_frequency(&self) -> BTreeMap<String, u32> {
        return self.words_frequency.clone();
    }
    pub fn get_win_rate(&self) -> f32 {
        if self.total_win == 0 {
            0 as f32
        } else {
            self.total_tries as f32 / self.total_win as f32
        }
    }
}

/// Return the words state
pub fn judge(guess: &str, answer: &str) -> [LetterState; 5] {
    use LetterState::*;
    let mut words_state = [R, R, R, R, R];
    let mut answer_state = [R, R, R, R, R];
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

/// Get answer word
pub fn get_answer_word(
    opt: &Opt,
    game_state: &mut GameData,
    day: usize,
    final_set: &Vec<String>,
) -> String {
    let mut answer_word: String = String::new();
    if opt.random == true {
        if opt.word.is_some() {
            unreachable!()
        }
        if opt.day.is_some() {
            let seed = opt.seed.unwrap_or(0);
            return final_set[ans(day, seed, final_set)].clone();
        }
        loop {
            answer_word = final_set[rand::thread_rng().gen_range(0..final_set.len())].clone();
            if !game_state.used_answer.contains(&answer_word) {
                game_state.push_used_answer(&answer_word);
                break;
            }
        }
    } else {
        if opt.day.is_some() || opt.seed.is_some() {
            unreachable!();
        }
        if opt.word.is_some() {
            answer_word = opt.word.clone().unwrap();
        } else {
            stdin().read_line(&mut answer_word).expect("");
        }
    }
    return answer_word;
}

/// Return the random index of FINAL
pub fn ans(d: usize, s: u64, final_set: &Vec<String>) -> usize {
    let mut rng = rand::rngs::StdRng::seed_from_u64(s);
    let mut answer_vec: Vec<usize> = (0..final_set.len()).collect();
    answer_vec.shuffle(&mut rng);
    return answer_vec[d - 1];
}

pub fn check_guess_in_difficult(guess: &String, last_word: &String, answer: &String) -> bool {
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
pub fn input_guess(
    opt: &Opt,
    last_word: &Option<String>,
    answer: &String,
    game_data: &mut GameData,
    acceptable_set: &Vec<String>,
) -> String {
    let mut guess = String::new();
    loop {
        guess.clear();
        stdin().read_line(&mut guess).expect("");
        guess = guess.trim().to_string().to_ascii_lowercase();
        // unqualified length
        if guess.chars().count() != 5 {
            println!("INVALID");
            continue;
        }
        // should be all letters
        if guess.chars().any(|x| !x.is_ascii_alphabetic()) {
            println!("INVALID");
            continue;
        }
        // not in word list
        if !acceptable_set.contains(&guess) {
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
    game_data.insert_word_frequency(&guess);
    return guess;
}

pub fn get_acceptable_set(opt: &Opt) -> Vec<String> {
    if opt.acceptable_set.is_none() {
        return crate::builtin_words::ACCEPTABLE
            .to_vec()
            .iter_mut()
            .map(|x| x.to_string())
            .collect();
    }

    let mut acceptable_set: Vec<String> = vec![];
    let file = File::open(opt.acceptable_set.clone().unwrap()).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line.unwrap();
        if word.trim().chars().count() != 5 {
            panic!();
        }
        if word.trim().chars().any(|x| !x.is_ascii_alphabetic()) {
            panic!();
        }
        if crate::builtin_words::ACCEPTABLE
            .binary_search(&word.as_str())
            .is_err()
        {
            panic!();
        }
        if acceptable_set.binary_search(&word).is_ok() {
            panic!();
        }
        acceptable_set.insert(
            acceptable_set.partition_point(|x| word > x.to_string()),
            word,
        );
    }
    return acceptable_set;
}

pub fn get_final_set(opt: &Opt, acceptable_set: &Vec<String>) -> Vec<String> {
    if opt.final_set.is_none() {
        return crate::builtin_words::FINAL
            .to_vec()
            .iter_mut()
            .map(|x| x.to_string())
            .collect();
    }
    let mut final_set: Vec<String> = vec![];
    let file = File::open(opt.final_set.clone().unwrap()).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line.unwrap();
        if word.trim().chars().count() != 5 {
            panic!();
        }
        if word.trim().chars().any(|x| !x.is_ascii_alphabetic()) {
            panic!();
        }
        if acceptable_set.binary_search(&word).is_err() {
            panic!();
        }
        if final_set.binary_search(&word).is_ok() {
            panic!();
        }
        final_set.insert(final_set.partition_point(|x| word > x.to_string()), word);
    }
    return final_set;
}
