use kalosm::language::*;
use crate::solution::file_library;

// different than v3 because it does not store a HashMap of active sessions
pub struct ChatbotV4 {
    model: Llama,
}

// stores the model
// sessions are handled later during chat_with_user
impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // make a filename from the username
        // each user gets their own file
        let filename = &format!("{}.txt", username);

        // creates a fresh chat session
        // checks whether there is already a saved session on disk for that user.
        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        // Try to load a saved session from file for this user.
        // If Some(session) comes back, restore it so the bot remembers past messages.
        // If None comes back (first time chatting), just keep the fresh session we made above.

        // load_chat_session_from_file(&filename) tries to read the user’s old session from the file
        // if SOME the code will rebuild the chat session using the saved session
        if let Some(session) = file_library::load_chat_session_from_file(&filename) {
            chat_session = self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate")
                .with_session(session);
        }

        // if none, there is no previous file so the fresh session stays

        // adds user's message
        // waits for model response
        let output = chat_session
            .add_message(message)
            .await;

        match output {
            Ok(response) => {
                // gets the updates session
                // save it to user's file
                // returns the response
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
            // will extract history
            Some(session) => {
                let history = session.history();
    
                history
                    .iter()
                    // skip the first message in history, which is likely the system prompt, so the returned history only includes the actual conversation content the user cares about.
                    .skip(1)
                    .map(|message| {
                        // The history messages were not directly in plain String form
                        // this code formats each message into text, extracts the content portion, and cleans up escaped characters like \n, \", and \\ so the final history is readable.
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
