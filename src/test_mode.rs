use crate::basic_function::*;

/// Test mode: tty false
pub fn test_mode(opt: &crate::Opt) {
    let mut game_data = GameData::new();

    let acceptable_set = get_acceptable_set(opt);
    let final_set = get_final_set(opt, &acceptable_set);

    let mut day = opt.day.unwrap_or(1);

    let mut state = crate::json_parse::State::load(opt, &mut game_data);

    loop {
        let answer_word: String = get_answer_word(opt, &mut game_data, day, &final_set);

        let mut keyboard = KeyboardState::new();
        let mut win = false;
        let mut tries: u32 = 0;
        let mut last_word: Option<String> = None; //difficult mode use
        let mut guesses: Vec<String> = Vec::new(); // save state use

        for _ in 0..6 {
            let guess = input_guess(
                &opt,
                &last_word,
                &answer_word,
                &mut game_data,
                &acceptable_set,
            );
            guesses.push(guess.clone());
            last_word = Some(guess.clone()); // this will be only used in next loop

            tries += 1;
            let word_state = judge(&guess.trim(), &answer_word.trim());
            keyboard.update(&guess, &word_state);
            for i in word_state.iter() {
                print!("{:?}", i);
            }
            print!(" {}\n", keyboard.to_string());
            if word_state.iter().all(|x| *x == LetterState::G) {
                win = true;
                break;
            }
        }

        if win == true {
            game_data.add_win();
            game_data.add_tries(tries);
            println!("CORRECT {}", tries);
        } else {
            game_data.add_lose();
            println!("FAILED {}", answer_word.trim().to_ascii_uppercase());
        }

        if opt.stats == true {
            println!(
                "{} {} {:.2}",
                game_data.get_win(),
                game_data.get_lose(),
                game_data.get_win_rate()
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
