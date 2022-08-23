use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{self, Block, Borders, Cell},
    Terminal,
};

pub fn interactive_mode(opt: &crate::Opt) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size1 = tui::layout::Rect {
            x: f.size().x,
            y: f.size().y,
            width: f.size().width / 2,
            height: f.size().height,
        };
        let block1 = Block::default()
            .title("Block")
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_style(Style::default().fg(Color::White))
            .border_type(tui::widgets::BorderType::Rounded)
            .style(Style::default().bg(Color::Black));
        let size2 = tui::layout::Rect {
            x: f.size().width / 2,
            y: f.size().y,
            width: f.size().width / 2,
            height: f.size().height,
        };
        let block2 = Block::default().title("title");

        f.render_widget(block1, size1);
        f.render_widget(block2, size2);
        loop {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Char('a') => {
                        let cell = widgets::Block::default().title("a");
                        f.render_widget(cell, Rect::new(5, 5, 1, 1));
                    }
                    _ => {}
                }
            }
        }
    })?;
    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
