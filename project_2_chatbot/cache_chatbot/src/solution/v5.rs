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
    
                let mut chat_session: Chat<Llama> = self.model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");
    
                if let Some(session) = file_library::load_chat_session_from_file(&filename) {
                    chat_session = self.model
                        .chat()
                        .with_system_prompt("The assistant will act like a pirate")
                        .with_session(session);
                }
    
                let output = chat_session.add_message(message).await;
    
                match output {
                    Ok(response) => {
                        {
                            let session = chat_session
                                .session()
                                .expect("session should exist");
                            file_library::save_chat_session_to_file(&filename, &session);
                        }
    
                        self.cache.insert_chat(username, chat_session);
                        return response.to_string();
                    }
                    Err(_) => {
                        return String::from("Sorry, I could not generate a response.");
                    }
                }
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
    
                let output = chat_session.add_message(message).await;
    
                match output {
                    Ok(response) => {
                        let session = chat_session
                            .session()
                            .expect("session should exist");
                        file_library::save_chat_session_to_file(&filename, &session);
                        return response.to_string();
                    }
                    Err(_) => {
                        return String::from("Sorry, I could not generate a response.");
                    }
                }
            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);
    
        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
    
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
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
    
                let history = chat_session
                    .session()
                    .expect("session should exist")
                    .history();
    
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