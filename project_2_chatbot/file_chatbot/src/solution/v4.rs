use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        // Try to load a saved session from file for this user.
        // If Some(session) comes back, restore it so the bot remembers past messages.
        // If None comes back (first time chatting), just keep the fresh session we made above.

        if let Some(session) = file_library::load_chat_session_from_file(&filename) {
            chat_session = self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate")
                .with_session(session);
        }

        let output = chat_session
            .add_message(message)
            .await;

        match output {
            Ok(response) => {
                let session = chat_session
                    .session()
                    .expect("session should exist");
                file_library::save_chat_session_to_file(&filename, &session);
                response.to_string()
            }
            Err(_) => String::from("Sorry, I could not generate a response."),
        }
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
    
        match file_library::load_chat_session_from_file(&filename) {
            None => Vec::new(),
            Some(session) => {
                let history = session.history();
    
                history
                    .iter()
                    .skip(1)
                    .map(|message| {
                        let text = format!("{:?}", message);
                        if let Some(start) = text.find("content: \"") {
                            let rest = &text[start + 10..];
                            if let Some(end) = rest.rfind('"') {
                                return rest[..end]
                                    .replace("\\n", "\n")
                                    .replace("\\\"", "\"")
                                    .replace("\\\\", "\\");
                            }
                        }
                        text
                    })
                    .collect()
            }
        }
    }
    
}