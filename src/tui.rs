/// A simple example demonstrating how to handle user input. This is
/// a bit out of the scope of the library as it does not provide any
/// input handling out of the box. However, it may helps some to get
/// started.
///
/// This is a very simple example:
///   * A input box always focused. Every character you type is registered
///   here
///   * Pressing Backspace erases a character
///   * Pressing Enter pushes the current input in the history of previous
///   messages
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use std::{
    error::Error,
    io::{self},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    basic_function::{
        check_guess_in_difficult, get_acceptable_set, get_final_set, judge, GameData,
        KeyboardState, LetterState,
    },
    builtin_words::{ACCEPTABLE, FINAL},
    Opt,
};

enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq)]
enum MessageMode {
    Valid,
    Invalid,
    Win,
    Fail,
    Input,
    Answer,
}

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    your_words: Vec<String>,

    message_mode: MessageMode,

    answer: String,

    keyboard: KeyboardState,

    word_state: Vec<[LetterState; 5]>,

    tries: u32,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            your_words: Vec::new(),
            message_mode: MessageMode::Input,
            answer: String::new(),
            keyboard: KeyboardState::new(),
            word_state: Vec::new(),
            tries: 0,
        }
    }
}

pub fn tui(opt: &Opt) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::default();
    if opt.random {
        app.message_mode = MessageMode::Input;
    } else {
        app.message_mode = MessageMode::Answer;
    }
    let res = run_app(&mut terminal, app, opt);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App, opt: &Opt) -> io::Result<()> {
    // Round loop
    loop {
        loop {
            terminal.draw(|f| ui(f, &app))?;

            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            let input = app
                                .input
                                .drain(..)
                                .collect::<String>()
                                .trim()
                                .to_string()
                                .to_ascii_lowercase();

                            match app.message_mode {
                                MessageMode::Fail | MessageMode::Win => {
                                    app.tries = 0;
                                    app.keyboard = KeyboardState::new();
                                    app.word_state.clear();
                                    app.your_words.clear();
                                    app.answer.clear();
                                    match input.as_str() {
                                        "Y" | "y" => app.message_mode = MessageMode::Answer,
                                        _ => return Ok(()),
                                    }
                                }
                                MessageMode::Answer => {
                                    let answer_word = input.clone();
                                    let mut valid = true;
                                    if answer_word.len() != 5 {
                                        valid = false;
                                    }
                                    if !answer_word.chars().all(|x| x.is_ascii_alphabetic()) {
                                        valid = false;
                                    }
                                    if !FINAL.binary_search(&answer_word.as_str()).is_err() {
                                        valid = false;
                                    }
                                    if valid {
                                        app.answer = answer_word.to_string();
                                    } else {
                                        app.answer = FINAL
                                            [rand::thread_rng().gen_range(0..FINAL.len())]
                                        .to_string();
                                    }
                                    app.message_mode = MessageMode::Input;
                                }
                                MessageMode::Input | MessageMode::Invalid | MessageMode::Valid => {
                                    if check_guess(&input, &mut app) {
                                        app.your_words.push(input.clone());
                                        app.input.clear();

                                        let word_state = judge(&input.trim(), &app.answer.trim());
                                        app.keyboard.update(&input, &word_state);

                                        app.word_state.push(word_state.clone());

                                        app.tries += 1;

                                        if word_state.iter().all(|x| *x == LetterState::G) {
                                            app.message_mode = MessageMode::Win;
                                        } else {
                                            if app.tries == 6 {
                                                app.message_mode = MessageMode::Fail;
                                            } else {
                                                app.message_mode = MessageMode::Valid;
                                            }
                                        }
                                        // continue;
                                    } else {
                                        app.message_mode = MessageMode::Invalid;
                                    }
                                }
                            }
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // The title block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Wordle Game")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, f.size());

    // Screen is divided into 3 chunks
    // chunk[0]: 1 row. Print input state
    // chunk[1]: 3 rows. Input chunk
    // chunk[2]: This will be divided into 2 or 3 chunks according to opt.hint
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    // chunk[2] is divided into two chunks. Each chunk has 50% size
    // display_chunks[0]: Print words' state
    // display_chunks[1]: This will be divided into two chunks or not according to opt.hint
    let display_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    //
    let output_hint_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(3)].as_ref())
        .split(display_chunks[1]);

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    let your_words: Vec<ListItem> = app
        .your_words
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![word_to_spans(i, m, &app)];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(your_words).block(Block::default().borders(Borders::ALL).title("Your Words"));
    f.render_widget(messages, display_chunks[0]);

    let message = Paragraph::new(app_to_string(&app))
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Message"));
    f.render_widget(message, output_hint_chunks[0]);

    let keyboard = Paragraph::new(keyboard_to_spans(&app))
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("KeyBoard"));
    f.render_widget(keyboard, output_hint_chunks[1]);
}

pub fn check_guess(guess: &String, app: &mut App) -> bool {
    let guess = guess.trim().to_string().to_ascii_lowercase();
    if guess.chars().count() != 5 {
        app.message_mode = MessageMode::Invalid;
        return false;
    }
    if guess.chars().any(|x| !x.is_ascii_alphabetic()) {
        app.message_mode = MessageMode::Invalid;
        return false;
    }
    if !ACCEPTABLE.contains(&guess.as_str()) {
        app.message_mode = MessageMode::Invalid;
        return false;
    }
    app.message_mode = MessageMode::Valid;
    true
}

fn app_to_string(app: &App) -> String {
    let res = match app.message_mode {
        MessageMode::Answer => {
            "Please input the answer \nIf you input invalid answer, it will be random".to_string()
        }
        MessageMode::Fail => {
            format!("You failed. Answer:{}\n Wanna try again? Y/N", app.answer).to_string()
        }
        MessageMode::Input => "Please input your guess".to_string(),
        MessageMode::Invalid => "Your guess is invalid. Try again".to_string(),
        MessageMode::Valid => "Valid guess".to_string(),
        MessageMode::Win => "You win! Wanna try again? Y/N".to_string(),
    };
    return res;
}

fn word_to_spans<'a>(i: usize, _m: &'a str, app: &'a App) -> Spans<'a> {
    let mut tmp: Vec<Span> = vec![];
    let word = app.your_words[i].clone();
    let state = app.word_state[i];
    let green = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);
    let red = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    let yellow = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    for (i, ch) in word.chars().enumerate() {
        match state[i] {
            LetterState::G => tmp.push(Span::styled(ch.to_string().to_ascii_uppercase(), green)),
            LetterState::R => tmp.push(Span::styled(ch.to_string().to_ascii_uppercase(), red)),
            LetterState::Y => tmp.push(Span::styled(ch.to_string().to_ascii_uppercase(), yellow)),
            _ => {}
        }
    }
    let res = Spans::from(tmp);
    return res;
}

fn keyboard_to_spans(app: &App) -> Vec<Spans> {
    let mut res: Vec<Spans> = vec![];

    let mut line1: Vec<Span> = vec![];
    line1.push(char_to_span('q', app));
    line1.push(Span::raw(" "));
    line1.push(char_to_span('w', app));
    line1.push(Span::raw(" "));
    line1.push(char_to_span('e', app));
    line1.push(Span::raw(" "));
    line1.push(char_to_span('r', app));
    line1.push(Span::raw(" "));
    line1.push(char_to_span('t', app));
    line1.push(Span::raw(" "));
    line1.push(char_to_span('y', app));
    line1.push(Span::raw(" "));
    line1.push(char_to_span('u', app));
    line1.push(Span::raw(" "));
    line1.push(char_to_span('i', app));
    line1.push(Span::raw(" "));
    line1.push(char_to_span('o', app));
    line1.push(Span::raw(" "));
    line1.push(char_to_span('p', app));
    res.push(Spans::from(line1));

    let mut line2: Vec<Span> = vec![];
    line2.push(Span::raw(" "));
    line2.push(char_to_span('a', app));
    line2.push(Span::raw(" "));
    line2.push(char_to_span('s', app));
    line2.push(Span::raw(" "));
    line2.push(char_to_span('d', app));
    line2.push(Span::raw(" "));
    line2.push(char_to_span('f', app));
    line2.push(Span::raw(" "));
    line2.push(char_to_span('g', app));
    line2.push(Span::raw(" "));
    line2.push(char_to_span('h', app));
    line2.push(Span::raw(" "));
    line2.push(char_to_span('j', app));
    line2.push(Span::raw(" "));
    line2.push(char_to_span('k', app));
    line2.push(Span::raw(" "));
    line2.push(char_to_span('l', app));
    res.push(Spans::from(line2));

    let mut line3: Vec<Span> = vec![];
    line3.push(Span::raw("  "));
    line3.push(char_to_span('z', app));
    line3.push(Span::raw(" "));
    line3.push(char_to_span('x', app));
    line3.push(Span::raw(" "));
    line3.push(char_to_span('c', app));
    line3.push(Span::raw(" "));
    line3.push(char_to_span('v', app));
    line3.push(Span::raw(" "));
    line3.push(char_to_span('b', app));
    line3.push(Span::raw(" "));
    line3.push(char_to_span('n', app));
    line3.push(Span::raw(" "));
    line3.push(char_to_span('m', app));
    res.push(Spans::from(line3));

    return res;
}

fn char_to_span(ch: char, app: &App) -> Span {
    let index = (ch as u8 - 'a' as u8) as usize;
    let green = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);
    let red = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    let yellow = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let gray = Style::default()
        .fg(Color::Gray)
        .add_modifier(Modifier::BOLD);
    let res = match app.keyboard.keyboard_state[index] {
        LetterState::G => Span::styled(ch.to_ascii_uppercase().to_string(), green),
        LetterState::R => Span::styled(ch.to_ascii_uppercase().to_string(), red),
        LetterState::X => Span::styled(ch.to_ascii_uppercase().to_string(), gray),
        LetterState::Y => Span::styled(ch.to_ascii_uppercase().to_string(), yellow),
    };
    return res;
}
