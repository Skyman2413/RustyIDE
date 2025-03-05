use iced::{Application, Command, Element, Settings};
use iced::widget::{text_input};
use rusty_core::language::LanguageEngine;
use std::sync::Arc;

pub struct Gui {
    content: String,
    engine: Arc<Box<dyn LanguageEngine>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(String),
    CompletionsReceived(Vec<String>),
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
                engine: Arc::new(rusty_core::language::MockEngine::new()),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Rusty IDE".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TextChanged(text) => {
                self.content = text.clone();
                let text_clone = text.clone();
                let engine = Arc::clone(&self.engine);
                Command::perform(
                    async move {
                        engine.get_completions(&text_clone, 0).await
                    },
                    Message::CompletionsReceived
                )
            }
            Message::CompletionsReceived(completions) => {
                println!("Completions: {:?}", completions);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        text_input("Enter code...", &self.content)
            .on_input(Message::TextChanged)
            .into()
    }
}

pub fn run() {
    let settings = Settings::default();
    Gui::run(settings).expect("Failed to run GUI");
}