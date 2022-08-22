use crate::Opt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::{fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    total_rounds: Option<i32>,
    games: Option<Vec<Game>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    answer: String,
    guesses: Vec<String>,
}

impl State {
    pub fn load(
        opt: &Opt,
        total_win: &mut u32,
        total_lose: &mut u32,
        total_tries: &mut u32,
        used_answer: &mut Vec<String>,
        word_frequency: &mut BTreeMap<String, u32>,
    ) -> State {
        let file = File::open(opt.state.clone().unwrap()).unwrap();
        let reader = BufReader::new(file);
        let state: State = serde_json::from_reader(reader).unwrap();

        if state.games.is_some() {
            let games = state.games.clone().unwrap();
            for game in games.iter() {
                used_answer.push(game.answer.clone());
                let win = Some(&game.answer) == game.guesses.last();
                if win {
                    *total_win += 1;
                } else {
                    *total_lose += 1;
                }

                for guess in game.guesses.iter() {
                    *word_frequency
                        .entry(guess.to_string().to_ascii_lowercase())
                        .or_insert(0) += 1;
                    if win {
                        *total_tries += 1;
                    }
                }
            }
        }

        return state;
    }
    pub fn save(&self, opt: &Opt) {
        let mut write = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(opt.state.clone().unwrap())
            .unwrap();
        write
            .write_all(serde_json::to_string_pretty(self).unwrap().as_bytes())
            .unwrap();
    }
    pub fn add_game(&mut self, game: Game) {
        if self.total_rounds.is_none() {
            self.total_rounds = Some(1);
            self.games = Some(Vec::new());
        } else {
            *self.total_rounds.as_mut().unwrap() += 1;
        }
        let mut game = game.clone();
        game.answer = game.answer.to_ascii_uppercase();
        game.guesses = game
            .guesses
            .iter_mut()
            .map(|x| x.to_ascii_uppercase())
            .collect();
        self.games.as_mut().unwrap().push(game);
    }
}

impl Game {
    pub fn new(answer: String, guesses: Vec<String>) -> Game {
        Game { answer, guesses }
    }
}
