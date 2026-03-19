use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    sessions: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model: model,
            sessions: HashMap::new(),
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // If this is the first message from this user, create a new chat session for them
        if !self.sessions.contains_key(&username) {
            let chat_session = self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate");
            self.sessions.insert(username.clone(), chat_session);
        }
        // Retrieve the existing session for this user and add the message
        let session = self.sessions.get_mut(&username).unwrap();
        return session.add_message(message).await.unwrap().to_string();
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        match self.sessions.get(&username) {
            Some(chat) => {
                chat.session()
                    .unwrap()
                    .history()
                    .iter()
                    .map(|m| m.content().to_string())
                    .collect()
            }
            None => Vec::new(),
        }
    }
}