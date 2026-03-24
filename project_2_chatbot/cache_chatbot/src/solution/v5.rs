use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                // The cache has this chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");

            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                // Not in cache, so load the session from file instead
                // If there's no file either, return empty history
                match file_library::load_chat_session_from_file(filename) {
                    None => {
                        return Vec::new();
                    }
                    Some(session) => {
                        // Build a fresh Chat, restore the saved session into it,
                        // then add it to the cache so future calls are fast
                        let chat_session = self.model
                            .chat()
                            .with_system_prompt("The assistant will act like a pirate")
                            .with_session(session);
                        self.cache.insert_chat(username.clone(), chat_session);

                        // Now get it back out of the cache and read its history
                        let chat_session = self.cache.get_chat(&username).unwrap();
                        return chat_session.session().unwrap()
                            .history()
                            .into_iter()
                            .map(|msg| msg.content().to_string())
                            .collect();
                    }
                }
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                // Already in cache, just read the history from the chat session directly
                return chat_session.session().unwrap()
                    .history()
                    .into_iter()
                    .map(|msg| msg.content().to_string())
                    .collect();
            }
        }
    }
}