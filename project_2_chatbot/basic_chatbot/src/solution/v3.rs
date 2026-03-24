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

            // Make sure you initialize your struct members here
            model: model,
            sessions: HashMap::new(),
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
        // If this is the first time we see this user, we need to create a new chat session for them
        // If this is the first message from this user, create a new chat session for them
        if !self.sessions.contains_key(&username) {
            let chat_session = self
                .model
                .chat()
                .with_system_prompt("The assistant will act like a pirate");
            self.sessions.insert(username.clone(), chat_session);
        }


        let chat_session = self.sessions.get_mut(&username).unwrap();
        let output = chat_session.add_message(message).await;
        
        match output {
            Ok(response) => response.to_string(),
            Err(_) => String::from("Sorry, I could not generate a response."),
        }
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!
        match self.sessions.get(&username) {
            Some(chat_session) => {
                let history = chat_session
                    .session()
                    .expect("session should exist")
                    .history();

                    history
                        .iter()
                        .map(|message| format!("{:?}", message))
                        .collect()
            }
            None => Vec::new(), // If there is no chat session for this user, return an empty history
        }
    }
}