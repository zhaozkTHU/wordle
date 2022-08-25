use std::{
    fs::{self},
    io::Write,
};

use indicatif::ParallelProgressIterator;
use rand::seq::SliceRandom;
use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    basic_function::{judge, LetterState},
    builtin_words::{ACCEPTABLE, FINAL},
    solver::{self, filter, solver},
};

pub fn calculate_average_answer() {
    let acceptable_set: Vec<String> = ACCEPTABLE
        .to_vec()
        .iter_mut()
        .map(|x| x.to_string())
        .collect();
    let final_set: Vec<String> = FINAL.to_vec().iter_mut().map(|x| x.to_string()).collect();

    let res: Vec<i32> = final_set
        .par_iter()
        .progress_count(final_set.len() as u64)
        .map(|answer| {
            let mut filtered_answer: Vec<usize> = (0..acceptable_set.len()).collect();
            let mut guess = "tares".to_string();
            let mut tries = 0;
            for _ in 0..6 {
                tries += 1;
                let guess_state = judge(&guess, answer);

                if guess_state.iter().all(|x| *x == LetterState::G) {
                    break;
                }

                filtered_answer = filter(&guess, &guess_state, &acceptable_set, &filtered_answer);
                guess = solver(&acceptable_set, &filtered_answer)[0].0.clone();
            }
            let mut file = fs::OpenOptions::new()
                .append(true)
                .write(true)
                .open("answer")
                .unwrap();
            file.write_all(format!("{} {}\n", answer, tries).as_bytes())
                .unwrap();
            tries
        })
        .collect();

    let mut average = 0.0;
    for i in res.iter() {
        average += *i as f64;
    }
    average /= res.len() as f64;
    let mut file = fs::OpenOptions::new()
        .append(true)
        .write(true)
        .open("answer")
        .unwrap();

    file.write_all((format!("{}\n", average)).as_bytes())
        .unwrap();
    for i in res.iter().enumerate() {
        file.write_all(format!("{} {}\n", &final_set[i.0], *i.1).as_bytes())
            .unwrap();
    }
}

pub fn calculate_average_begin() {
    let acceptable_set: Vec<String> = ACCEPTABLE
        .to_vec()
        .iter_mut()
        .map(|x| x.to_string())
        .collect();
    let final_set: Vec<String> = FINAL.to_vec().iter_mut().map(|x| x.to_string()).collect();

    let mut res: Vec<(String, i32)> = final_set.iter().map(|x| (x.clone(), 0)).collect();
    let len = res.len();

    res.par_iter_mut().progress_count(len as u64).for_each(|a| {
        let first_guess = a.0.clone();

        for _ in 0..10 {
            let answer_word = final_set.choose(&mut rand::thread_rng()).unwrap().clone();

            let mut guess = first_guess.to_string();
            let mut filtered_answer: Vec<usize> = (0..acceptable_set.len()).collect();

            for _ in 0..6 {
                a.1 += 1;
                let guess_state = judge(&guess, &answer_word);

                if guess_state.iter().all(|x| *x == LetterState::G) {
                    break;
                }

                filtered_answer = filter(&guess, &guess_state, &acceptable_set, &filtered_answer);
                guess = solver::solver(&acceptable_set, &filtered_answer)[0]
                    .clone()
                    .0;
            }
        }
        let mut fils1 = fs::OpenOptions::new()
            .append(true)
            .write(true)
            .open("tmp")
            .unwrap();
        fils1
            .write_all((format!("{} {}\n", a.0, a.1)).as_bytes())
            .unwrap();
    });

    res.sort_by(|a, b| a.1.cmp(&b.1));
    let mut average = 0.0;

    let mut file = fs::OpenOptions::new()
        .append(true)
        .write(true)
        .open("res")
        .unwrap();
    for i in res.iter() {
        file.write_all((format!("{} {}\n", i.0, i.1)).as_bytes())
            .unwrap();
        average += i.1 as f64;
    }
    file.write_all((format!("{}", average / (res.len() * 2) as f64)).as_bytes())
        .unwrap();
}
