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
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            your_words: Vec::new(),
            message_mode: MessageMode::Input,
        }
    }
}

pub fn main(opt: &Opt) -> Result<(), Box<dyn Error>> {
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
    let mut game_data = GameData::new();

    let acceptable_set = get_acceptable_set(opt);
    let final_set = get_final_set(opt, &acceptable_set);

    let mut day = opt.day.unwrap_or(1);

    let mut state = crate::json_parse::State::load(opt, &mut game_data);

    // Round loop
    loop {
        let mut answer_word = String::new();

        let mut keyboard = KeyboardState::new();
        let mut win = false;
        let mut tries: u32 = 0;
        let mut last_word: Option<String> = None;
        let mut guesses: Vec<String> = Vec::new();

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
                            let input: String = app.input.drain(..).collect();

                            match app.message_mode {
                                MessageMode::Fail | MessageMode::Win => {
                                    let y = "Y".to_string();
                                    match input {
                                        y => app.message_mode = MessageMode::Answer,
                                        _ => return Ok(()),
                                    }
                                }
                                MessageMode::Answer => {
                                    answer_word = input.clone();
                                }
                                MessageMode::Input | MessageMode::Invalid | MessageMode::Valid => {
                                    if check_guess(
                                        opt,
                                        &input,
                                        &last_word,
                                        &answer_word,
                                        &mut game_data,
                                        &acceptable_set,
                                        &mut app,
                                    ) {
                                        app.your_words.push(input.clone());
                                        app.input.clear();

                                        guesses.push(input.clone());
                                        last_word = Some(input.clone());

                                        let word_state = judge(&input.trim(), &answer_word.trim());
                                        keyboard.update(&input, &word_state);

                                        tries += 1;

                                        if word_state.iter().all(|x| *x == LetterState::G) {
                                            app.message_mode = MessageMode::Win;
                                            win = true;
                                        } else {
                                            if tries == 6 {
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
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(your_words).block(Block::default().borders(Borders::ALL).title("Your Words"));
    f.render_widget(messages, display_chunks[0]);

    let message = Paragraph::new("INVALID")
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Message"));
    f.render_widget(message, output_hint_chunks[0]);

    let keyboard = Paragraph::new("UNENABLED")
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("KesyBoard"));
    f.render_widget(keyboard, output_hint_chunks[1]);
}

pub fn check_guess(
    opt: &Opt,
    guess: &String,
    last_word: &Option<String>,
    answer: &String,
    game_data: &mut GameData,
    accptable_set: &Vec<String>,
    app: &mut App,
) -> bool {
    let guess = guess.trim().to_string().to_ascii_lowercase();
    if guess.chars().count() != 5 {
        app.message_mode = MessageMode::Invalid;
        return false;
    }
    if guess.chars().any(|x| !x.is_ascii_alphabetic()) {
        app.message_mode = MessageMode::Invalid;
        return false;
    }
    if !accptable_set.contains(&guess) {
        app.message_mode = MessageMode::Invalid;
        return false;
    }
    if opt.difficult {
        if last_word.is_some() {
            if !check_guess_in_difficult(&guess, last_word.clone().as_mut().unwrap(), answer) {
                app.message_mode = MessageMode::Invalid;
                return false;
            }
        }
    }
    app.message_mode = MessageMode::Valid;
    true
}
