use kalosm::language::*;

// only stores LLM model
// does not store a chat session, history, usernames, files, or cache so we know that v1 doesn't store memory
#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama,
}

impl ChatbotV1 {
    #[allow(dead_code)]
    // Initializes the chatbot with a Llama model so we can use that model later to start chats.
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { model: model };
    }

    
    #[allow(dead_code)]
    // takes in &mut self so it can use the chatbot object
    // takes in message: String which is the user's message
    // returns a string
    // async because generating a response from the model takes time, so Rust needs to wait for the result with .await
    pub async fn chat_with_user(&mut self, message: String) -> String {
        
        // 1. takes stored model 2. starts a new chat session 3. gives it a system prompt so the assistant acts like a pirate
        //every time the function is called, a new chat_session is created from scratch - so after the function ends, the chat session will disappear
        // local variable 
        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        // adds the user’s input to the session and asks the model to generate a reply in that same session context
        let output = chat_session.add_message(message).await;

        // handles output cases
        match output {
            // If the model successfully returns a response, return it
            Ok(response) => response,

            // If something goes wrong, return a fallback error message
            Err(_) => String::from("Sorry, I could not generate a response."),
        }
    }
}
