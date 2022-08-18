mod builtin_words;
use console;
use rand::{self, Rng};
use std::io::{self, Write};

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);

    if is_tty {
        println!(
            "I am in a tty. Please print {}!",
            console::style("colorful characters").bold().blink().blue()
        );
    } else {
        println!("I am not in a tty. Please print according to test requirements!");
    }

    if is_tty {
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();
    }
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    println!("Welcome to wordle, {}!", line.trim());

    // example: print arguments
    print!("Command line arguments: ");
    for arg in std::env::args() {
        print!("{} ", arg);
    }
    println!("");
    // TODO: parse the arguments in `args`

    if is_tty {
        use builtin_words::{ACCEPTABLE, FINAL};
        let answer_word: String = FINAL[rand::thread_rng().gen_range(0..FINAL.len())].to_string();
        println!("{}", &answer_word);
        let mut win = false;
        for _ in 0..6 {
            let mut guess_words: String;
            let mut correct = 0;
            loop {
                guess_words = text_io::read!();
                if guess_words.len() < 5 {
                    print!("{}", console::style("Not enough letters").bold().red());
                    continue;
                }
                if guess_words.len() > 5 {
                    print!("{}", console::style("Too many letters.").bold().red());
                }
                if !ACCEPTABLE.contains(&guess_words.as_str()) {
                    println!("{}", console::style("Not in word list").bold().red());
                    continue;
                }
                break;
            }
            for i in 0..guess_words.len() {
                let answer_vec: Vec<char> = answer_word.clone().chars().collect();
                let guess_vec: Vec<char> = guess_words.clone().chars().collect();
                if answer_vec.contains(&guess_vec[i]) {
                    if answer_vec[i] == guess_vec[i] {
                        print!("{}", console::style(&guess_vec[i]).green());
                        correct += 1;
                    } else {
                        print!("{}", console::style(&guess_vec[i]).yellow());
                    }
                } else {
                    print!("{}", console::style(&guess_vec[i]).red());
                }
            }
            print!("\n");
            if correct == 5 {
                println!(
                    "{}",
                    console::style("Congratulations, you find the answer!")
                        .green()
                        .bold()
                );
                win = true;
                break;
            }
        }
        if win == false {
            println!(
                "{}",
                console::style("Sorry, you lose the game.Please try again.")
                    .red()
                    .bold()
            )
        }
    } else {
    }

    Ok(())
}
