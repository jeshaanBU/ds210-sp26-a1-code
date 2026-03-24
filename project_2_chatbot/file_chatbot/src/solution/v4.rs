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
        if let Some(saved_session) = file_library::load_chat_session_from_file(filename) {
            chat_session = chat_session.with_session(saved_session);
        }

        // Send the user's message and get the bot's response
        let output = chat_session.add_message(message).await;

        // Save the updated session back to the file so history is kept for next time
        if let Ok(session) = chat_session.session() {
            file_library::save_chat_session_to_file(filename, &*session);
        }

        match output {
            Ok(response) => response,
            Err(_) => String::from("Sorry, I could not generate a response."),
        }
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => {
                // TODO: what should happen here?
                return Vec::new();
            }
        }
    }
}