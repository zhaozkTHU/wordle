use crate::basic_function::*;
use crate::Opt;
use colored::*;

pub fn interactive_mode(opt: &Opt) {
    let mut game_data = GameData::new();

    let acceptable_set = get_acceptable_set(opt);
    let final_set = get_final_set(opt, &acceptable_set);

    let mut day = opt.day.unwrap_or(1);

    let mut state = crate::json_parse::State::load(opt, &mut game_data);

    let mut hint_acceptable: Vec<usize> = (0..acceptable_set.len()).collect();

    loop {
        let hint = crate::solver::solver(&acceptable_set, &hint_acceptable);
        if opt.hint {
            print!("HINT: ");
            for i in hint.iter() {
                print!("{}", i.to_ascii_uppercase());
            }
            print!("\n");
        }
        if opt.word.is_none() && !opt.random {
            println!("{}", "Please input your answer:".bold());
        }
        let answer_word: String = get_answer_word(opt, &mut game_data, day, &final_set);

        let mut keyboard = KeyboardState::new();
        let mut win = false;
        let mut tries: u32 = 0;
        let mut last_word: Option<String> = None; //difficult mode use
        let mut guesses: Vec<String> = Vec::new(); // save state use

        for _ in 0..6 {
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

            let word_state = judge(&guess.trim(), &answer_word.trim());
            keyboard.update(&guess, &word_state);

            if !hint.contains(&guess) {
                hint_acceptable = (0..acceptable_set.len()).collect();
            } else {
                hint_acceptable =
                    crate::solver::filter(&guess, &word_state, &acceptable_set, &hint_acceptable);
            }

            tries += 1;
            for i in word_state.iter().enumerate() {
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
                            .green()
                    ),
                    _ => {
                        panic!();
                    }
                }
            }
            print!(" ");
            keyboard_to_color(&keyboard);

            if word_state.iter().all(|x| *x == LetterState::G) {
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
