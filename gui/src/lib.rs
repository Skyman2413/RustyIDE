use iced::{Application, Command, Element, Settings};
use core::language::LanguageEngine;

pub struct Gui {
    content: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Пока пусто, добавим позже
}
impl Application for Gui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Gui {
                content: "Hello, IDE!".to_string(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "RustyIDE".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        iced::widget::text(&self.content).into()
    }
}

pub fn run() {
    let settings = Settings::default();
    Gui::run(settings).expect("Failed to run GUI");
}