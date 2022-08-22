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

pub fn parse_config(config_path: &Option<String>, opt: &crate::Opt) -> crate::Opt {
    let file = File::open(config_path.clone().unwrap()).unwrap();
    let reader = BufReader::new(file);
    let config: serde_json::Value = serde_json::from_reader(reader).unwrap();
    Opt {
        word: if opt.word.is_some() {
            opt.word.clone()
        } else {
            if config["word"].is_string() {
                Some(config["word"].as_str().unwrap().to_string())
            } else {
                None
            }
        },

        random: if opt.random == true {
            true
        } else {
            if config["random"].is_boolean() {
                config["random"].as_bool().unwrap()
            } else {
                false
            }
        },

        difficult: if opt.difficult == true {
            true
        } else {
            if config["difficult"].is_boolean() {
                config["difficult"].as_bool().unwrap()
            } else {
                false
            }
        },

        stats: if opt.stats == true {
            true
        } else {
            if config["stats"].is_boolean() {
                config["stats"].as_bool().unwrap()
            } else {
                false
            }
        },

        seed: if opt.seed.is_some() {
            opt.seed.clone()
        } else {
            if config["seed"].is_u64() {
                Some(config["seed"].as_u64().unwrap())
            } else {
                None
            }
        },

        day: if opt.day.is_some() {
            opt.day.clone()
        } else {
            if config["day"].is_i64() {
                Some(config["day"].as_i64().unwrap() as usize)
            } else {
                None
            }
        },

        final_set: if opt.final_set.is_some() {
            opt.final_set.clone()
        } else if config["final_set"].is_string() {
            Some(config["final_set"].as_str().unwrap().to_string())
        } else {
            None
        },

        acceptable_set: if opt.acceptable_set.is_some() {
            opt.acceptable_set.clone()
        } else if config["acceptable_set"].is_string() {
            Some(config["acceptable_set"].as_str().unwrap().to_string())
        } else {
            None
        },

        state: if opt.state.is_some() {
            opt.state.clone()
        } else if config["state"].is_string() {
            Some(config["state"].as_str().unwrap().to_string())
        } else {
            None
        },

        config: Some(config_path.clone().unwrap()),
    }
}
