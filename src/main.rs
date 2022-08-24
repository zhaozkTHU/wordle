mod all_state;
mod basic_function;
mod builtin_words;
mod interactive_mode;
mod json_parse;
mod solver;
mod test_mode;
mod tui;

use std::io::stdin;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// Specify word
    #[structopt(short, long)]
    word: Option<String>,

    /// Random answer word
    #[structopt(short, long)]
    random: bool,

    /// Difficult mode
    #[structopt(short = "D", long)]
    difficult: bool,

    /// Print stats after game
    #[structopt(short = "t", long)]
    stats: bool,

    /// Seed in difficult mode
    #[structopt(short, long)]
    seed: Option<u64>,

    /// Start in nth day
    #[structopt(short, long)]
    day: Option<usize>,

    /// Get final set from file
    #[structopt(short, long)]
    final_set: Option<String>,

    /// Get acceptable set from file
    #[structopt(short, long)]
    acceptable_set: Option<String>,

    /// Save and load in a json file
    #[structopt(short = "S", long)]
    state: Option<String>,

    /// Load configs from a json file
    #[structopt(short, long)]
    config: Option<String>,

    /// Enable solver
    #[structopt(short, long)]
    hint: bool,
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut opt = Opt::from_args();
    if opt.config.is_some() {
        opt = json_parse::parse_config(&opt.config, &opt);
    }
    let is_tty = atty::is(atty::Stream::Stdout);

    // if is_tty {
    //     println!(
    //         "I am in a tty. Please print {}!",
    //         console::style("colorful characters").bold().blink().blue()
    //     );
    // } else {
    //     // println!("I am not in a tty. Please print according to test requirements!");
    // }

    // if is_tty {
    //     print!("{}", console::style("Your name: ").bold().red());
    //     io::stdout().flush().unwrap();
    // }
    // let mut line = String::new();
    // io::stdin().read_line(&mut line)?;
    // println!("Welcome to wordle, {}!", line.trim());

    // example: print arguments
    // print!("Command line arguments: ");
    // println!("");
    // TODO: parse the arguments in `args`

    if is_tty {
        let mut tmp = String::new();
        println!("Do you want to play in TUI? Y/N");
        stdin().read_line(&mut tmp)?;
        match tmp.trim() {
            "Y" => {
                crate::tui::tui(&opt)?;
            }
            "N" => interactive_mode::interactive_mode(&opt),
            _ => return Ok(()),
        }
    } else {
        test_mode::test_mode(&opt);
    }

    Ok(())
}
