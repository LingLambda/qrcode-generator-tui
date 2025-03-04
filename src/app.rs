use tui_textarea::TextArea;

pub enum CurrentScreen {
    Main,
    Exiting,
}

pub enum SaveOption {
    SaveAndOpen,
    SaveNotOpen,
    DontSave,
}

pub struct App<'a> {
    pub char_count: i64,
    pub text_area: TextArea<'a>,
    pub current_screen: CurrentScreen,
}

impl App<'_> {
    pub fn new() -> Self {
        App {
            char_count: 0,
            text_area: TextArea::default(),
            current_screen: CurrentScreen::Main,
        }
    }
}
