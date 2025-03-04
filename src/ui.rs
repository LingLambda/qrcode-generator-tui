use crate::app::App;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::{Style, Text};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());
    let mut char_count_text = String::from("字数：");
    let count = app.char_count.to_string();
    char_count_text.push_str(count.as_str());
    let para = Paragraph::new(Text::styled(char_count_text, Style::default()))
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(&app.text_area, chunks[0]);
    frame.render_widget(para, chunks[1]);
}
