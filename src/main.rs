mod app;
mod command;
mod generator;
mod ui;

use crate::app::App;
use crate::command::command;
use crate::generator::generator_qrcode;
use crate::ui::ui;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
use std::io;
use tui_textarea::{Input, Key};

fn main() -> io::Result<()> {
    command();

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut app = App::new();
    app.text_area.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("在下方输入文本"),
    );
    loop {
        let mut text = String::new();
        for str in app.text_area.lines() {
            text.push_str(str);
        }
        app.char_count = text.len() as i64;
        term.draw(|f| ui(f, &mut app))?;
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            input => {
                app.text_area.input(input);
            }
        }
    }
    disable_raw_mode()?;
    crossterm::execute!(term.backend_mut(), LeaveAlternateScreen)?;
    term.show_cursor()?;
    let mut text = String::new();
    for str in app.text_area.lines() {
        text.push_str(str);
        text.push_str("\r\n")
    }
    generator_qrcode(text);
    Ok(())
}
