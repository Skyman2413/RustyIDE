use async_trait::async_trait;

#[async_trait]
pub trait LanguageEngine: Send + Sync {
    async fn get_syntax_highlighting(&self, code: &str) -> Vec<()>;
    async fn get_completions(&self, code: &str, position: usize) -> Vec<String>;
}

pub struct MockEngine;

impl MockEngine {
    pub fn new() -> Box<dyn LanguageEngine> {
        Box::new(MockEngine)
    }
}

#[async_trait]
impl LanguageEngine for MockEngine {
    async fn get_syntax_highlighting(&self, _code: &str) -> Vec<()> { vec![] }
    async fn get_completions(&self, code: &str, _position: usize) -> Vec<String> {
        if code.starts_with("pr") {
            vec!["print".to_string(), "process".to_string()]
        } else {
            vec!["Это тест, б!?ть".to_string()]
        }
    }
}