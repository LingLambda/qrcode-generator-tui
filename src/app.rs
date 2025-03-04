use tui_textarea::TextArea;

#[derive(Debug, Default)]
pub struct App<'a> {
    pub char_count:i64,
    pub text_area: TextArea<'a>
}

impl App<'_> {
    pub fn new() -> Self {
        App{
            char_count:0,
            text_area:TextArea::default()
        }
    }
}
