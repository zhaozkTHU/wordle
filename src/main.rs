mod basic_function;
mod builtin_words;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short, long)]
    word: Option<String>,

    #[structopt(short, long)]
    random: bool,

    #[structopt(short = "D", long)]
    difficult: bool,

    #[structopt(short = "t", long)]
    stats: bool,

    #[structopt(short, long)]
    seed: Option<u64>,

    #[structopt(short, long)]
    day: Option<usize>,

    #[structopt(short, long, parse(from_os_str))]
    final_set: Option<PathBuf>,

    #[structopt(short, long, parse(from_os_str))]
    acceptable_set: Option<PathBuf>,

    #[structopt(short = "S", long)]
    state: bool,
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
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
        basic_function::test_mode(&opt);
    } else {
        basic_function::test_mode(&opt);
    }

    Ok(())
}
