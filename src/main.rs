mod app;
mod command;
mod generator;
mod ui;

use crate::app::{App, CurrentScreen, SaveOption};
use crate::command::command;
use crate::generator::generator_qrcode;
use crate::ui::ui;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::style::{Color, Style};
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
    app.text_area
        .set_line_number_style(Style::default().fg(Color::Rgb(70,70,70)).bg(Color::Gray));

    let save_option:SaveOption;
    loop {
        let mut text = String::new();
        for str in app.text_area.lines() {
            text.push_str(str);
        }
        app.char_count = text.len() as i64;
        term.draw(|f| ui(f, &mut app))?;
        match app.current_screen {
            CurrentScreen::Main=>{
                match crossterm::event::read()?.into() {
                    Input { key: Key::Esc, .. } => { 
                        app.current_screen=CurrentScreen::Exiting;
                    },
                    input => {
                        app.text_area.input(input);
                    }
                }
            }
            CurrentScreen::Exiting=>{
                match crossterm::event::read()?.into() {
                    Input { key: Key::Esc, .. } => { 
                        app.current_screen = CurrentScreen::Main;
                    }
                    Input { key: Key::Char('Y')|Key::Char('y'),.. }=>{
                        save_option = SaveOption::SaveAndOpen;
                        break;
                    },
                    Input { key: Key::Char('S')|Key::Char('s'),.. }=>{
                        save_option = SaveOption::SaveNotOpen;
                        break;
                    },
                    Input { key: Key::Char('N')|Key::Char('n'),.. }=>{
                        save_option = SaveOption::DontSave;
                        break;
                    },
                    _=>{}
                }
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
    generator_qrcode(text,save_option);
    Ok(())
}
