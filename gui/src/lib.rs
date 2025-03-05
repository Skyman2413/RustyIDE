use iced::{Application, Element, Settings, Command};
use rusty_core::language::LanguageEngine;

pub struct Gui {
    content: String,
    engine: Box<dyn LanguageEngine>
}

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(String),
    CompetitionsGot(Vec<String>)
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
                engine: rusty_core::language::MockEngine::new()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "RustyIDE".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TextChanged(text) => {
                self.content = text.clone();

                let text_clone = text.clone();
                let engine = self.engine.as_ref();
                Command::perform(
                    async move {
                        engine.get_completions(&text_clone, 0).await
                    },
                    |completions| Message::CompetitionsGot(completions)
                )
            }
            Message::CompetitionsGot(vector) => {
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        iced::widget::text_input("Input", &self.content)
            .on_input(|text| {
                Message::TextChanged(text)
            })
            .into()
    }
}

pub fn run() {
    let settings = Settings::default();
    Gui::run(settings).expect("Failed to run GUI");
}