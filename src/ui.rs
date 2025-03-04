use crate::app::{App, CurrentScreen};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style, Text};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

pub fn ui(frame: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::Main=>main_ui(frame,app),
        CurrentScreen::Exiting=>exit_ui(frame),
    }
}

fn main_ui(frame:&mut Frame,app: &mut App){
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

fn exit_ui(frame:&mut Frame){
    frame.render_widget(Clear, frame.area()); //清屏
    let popup_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::Gray));

    let exit_text = Text::styled(
        "保存二维码吗？\n   Y:保存并打开\n   S:保存但不要打开\n   N:不保存\n   ESC:取消",
        Style::default().fg(Color::Black),
    );
    // trim false 使得文本不会在超出边缘时被 截断/隐藏
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = chunk_exit_rect(frame.area());
    frame.render_widget(exit_paragraph, area);
}



fn chunk_exit_rect(rect: Rect) -> Rect {
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(100 / 3),
            Constraint::Percentage(100 / 3),
            Constraint::Percentage(100 / 3),
        ])
        .split(rect);

    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(100 / 3),
            Constraint::Percentage(100 / 3),
            Constraint::Percentage(100 / 3),
        ])
        .split(rects[1]);

    rects[1]
}
