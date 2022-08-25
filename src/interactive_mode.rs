use crate::basic_function::*;
use crate::solver::filter;
use crate::Opt;
use colored::*;

pub fn interactive_mode(opt: &Opt) {
    let mut game_data = GameData::new();

    let acceptable_set = get_acceptable_set(opt);
    let final_set = get_final_set(opt, &acceptable_set);

    let mut day = opt.day.unwrap_or(1);

    let mut state = crate::json_parse::State::load(opt, &mut game_data);

    loop {
        let mut filtered_answer: Vec<usize> = (0..acceptable_set.len()).collect();
        if opt.word.is_none() && !opt.random {
            println!("{}", "Please input your answer:".bold());
        }
        let answer_word: String = get_answer_word(opt, &mut game_data, day, &final_set);

        let mut keyboard = KeyboardState::new();
        let mut win = false;
        let mut tries: u32 = 0;
        let mut last_word: Option<String> = None; //difficult mode use
        let mut guesses: Vec<String> = Vec::new(); // save state use
        let mut guess_states: Vec<[LetterState; 5]> = Vec::new();

        for round in 0..6 {
            let mut hint: Vec<(String, f64)> = vec![
                ("tares".to_string(), 6.19),
                ("lares".to_string(), 6.15),
                ("rales".to_string(), 6.11),
                ("rates".to_string(), 6.10),
                ("teras".to_string(), 6.08),
            ];
            if opt.hint {
                if round != 0 {
                    hint = crate::solver::solver(&acceptable_set, &filtered_answer);
                }
                print!("HINT: ");
                for i in hint.iter() {
                    print!(
                        "{} {} ",
                        i.0.to_ascii_uppercase().purple(),
                        format!("{:.2}", i.1).purple()
                    );
                }
                print!("\n");
            }
            println!("{}", "Please input your guess:".bold());
            let guess = input_guess(
                &opt,
                &last_word,
                &answer_word,
                &mut game_data,
                &acceptable_set,
            );

            guesses.push(guess.clone());
            last_word = Some(guess.clone()); // this will be only used in next loop

            let guess_state = judge(&guess.trim(), &answer_word.trim());
            guess_states.push(guess_state.clone());
            keyboard.update(&guess, &guess_state);

            if opt.hint {
                for i in guesses.iter().zip(guess_states.iter()) {
                    filtered_answer = filter(i.0, i.1, &acceptable_set, &filtered_answer)
                }
            }

            tries += 1;
            for i in guess_state.iter().enumerate() {
                match i.1 {
                    LetterState::G => print!(
                        "{}",
                        guess
                            .chars()
                            .nth(i.0)
                            .unwrap()
                            .to_ascii_uppercase()
                            .to_string()
                            .bold()
                            .green()
                    ),
                    LetterState::R => print!(
                        "{}",
                        guess
                            .chars()
                            .nth(i.0)
                            .unwrap()
                            .to_ascii_uppercase()
                            .to_string()
                            .bold()
                            .red()
                    ),
                    LetterState::Y => print!(
                        "{}",
                        guess
                            .chars()
                            .nth(i.0)
                            .unwrap()
                            .to_ascii_uppercase()
                            .to_string()
                            .bold()
                            .yellow()
                    ),
                    _ => {
                        panic!();
                    }
                }
            }
            print!(" ");
            keyboard_to_color(&keyboard);

            if guess_state.iter().all(|x| *x == LetterState::G) {
                win = true;
                break;
            }
        }

        if win == true {
            game_data.add_win();
            game_data.add_tries(tries);
            println!(
                "{} {}",
                "CORRECT".green().bold(),
                tries.to_string().bold().bright_yellow()
            );
        } else {
            game_data.add_lose();
            println!(
                "{} {}",
                "FAILED".red().bold(),
                answer_word.trim().to_ascii_uppercase().bold().green()
            );
        }

        if opt.stats == true {
            println!(
                "WIN:{} LOSE:{} WIN_RATE:{:.2}",
                game_data.get_win().to_string().green().bold(),
                game_data.get_lose().to_string().red().bold(),
                game_data.get_win_rate().to_string().bold()
            );
            let mut tmp = game_data.get_words_frequency();
            for i in 0..5 {
                if tmp.is_empty() {
                    break;
                }
                if i != 0 {
                    print!(" ");
                }
                let max = tmp.iter().rev().max_by_key(|x| x.1).unwrap();
                let a = max.0.clone();
                print!("{} {}", max.0.to_ascii_uppercase(), max.1);
                tmp.remove(&a);
            }
            print!("\n");
        }

        if opt.state.is_some() {
            let mut tmp = state.unwrap();
            tmp.add_game(crate::json_parse::Game::new(answer_word.clone(), guesses));
            tmp.save(opt);
            state = Some(tmp);
        }

        println!("{}", "Do you want to play again? Y/N".bold());
        if opt.word.is_none() || opt.stats {
            let mut again = String::new();
            let bytes = std::io::stdin().read_line(&mut again).unwrap();
            if again.trim() == "N" || bytes == 0 || again == "\n" {
                break;
            }
            if again.trim() == "Y" {
                day += 1;
                continue;
            }
        }
        break;
    } // Loop end
}

fn keyboard_to_color(keyboard: &KeyboardState) {
    for i in 0..26 {
        match keyboard.keyboard_state[i] {
            LetterState::X => print!("{}", (('A' as u8 + i as u8) as char).to_string().bold()),
            LetterState::G => print!(
                "{}",
                (('A' as u8 + i as u8) as char).to_string().bold().green()
            ),
            LetterState::Y => print!(
                "{}",
                (('A' as u8 + i as u8) as char).to_string().bold().yellow()
            ),
            LetterState::R => print!(
                "{}",
                (('A' as u8 + i as u8) as char).to_string().bold().red()
            ),
        }
    }
    print!("\n");
}
