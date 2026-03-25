use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    // adds memory by storing the session in the structure. Each new message is added to the same existing conversation
    // uses HashMap because it will look up a username in the map and then use that session
    sessions: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {

        // sessions will be created once a user sends their first message
        return ChatbotV3 {

            // creates model
            model: model,

            // creates an empty HashMap
            sessions: HashMap::new(),
        };
    }

    #[allow(dead_code)]
    // function takes two inputs, 1. username 2. message
    // this makes sure the program knows which user's history to use
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Checks if user already has a session

        // if username is NOT in the map, it creates a brand new chat session for the user
        if !self.sessions.contains_key(&username) {
            let chat_session = self
                .model
                .chat()
                .with_system_prompt("The assistant will act like a pirate");
            // inserts the new chat session into self.sessions so new users will get a fresh chat
            // uses username.clone() b/c inserting into the HashMap will take ownership of the key
            //We still need the username afterward to retrieve the mutable session
            self.sessions.insert(username.clone(), chat_session);
        }

        // retrieves a mutable reference to the user's chat session
        // get_mut b/c we need mutable access to add a new message to session history
        // unwrap is ok because we know the session exists or it was inserted so it will not crash
        let chat_session = self.sessions.get_mut(&username).unwrap();

        // adds users messages to their own chat session, not a shared one
        // response will be based on their previous history 
        let output = chat_session.add_message(message).await;

        
        match output {
            // If the model successfully returns a response, return it
            Ok(response) => response,

            // If something goes wrong, return a fallback error message
            Err(_) => String::from("Sorry, I could not generate a response."),
        }
    }

    #[allow(dead_code)]
    // looks up the chat session for a given username
    // converts that session's message history into a vector of strings
    // if the user doesn't have a session yet, it will return an empty vector
    pub fn get_history(&self, username: String) -> Vec<String> {
        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!

        // looks in the HashMap using the username
        match self.sessions.get(&username) {
            // user exists
            Some(chat_session) => {
                // gets access to the underlying session and its history
                // history has all of the specific users' conversation messages that have been stored in the chat session so far
                let history = chat_session.session().unwrap().history();

                // maps over the history messages
                // extracts the contents of each message and converts it to a string
                // collects the results into a Vec<String> 
                history
                    .into_iter()
                    .map(|msg| msg.content().to_string())
                    .collect()
            }

            // no chat session for this user, return an empty history
            // won't crash if empty
            None => Vec::new(),
        }
    }
}
