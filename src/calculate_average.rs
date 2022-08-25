use std::{
    fs::{self},
    io::Write,
};

use indicatif::ParallelProgressIterator;
use rand::seq::SliceRandom;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    basic_function::{judge, LetterState},
    builtin_words::{ACCEPTABLE, FINAL},
    solver::{self, filter},
};

pub fn calculate_average() {
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
