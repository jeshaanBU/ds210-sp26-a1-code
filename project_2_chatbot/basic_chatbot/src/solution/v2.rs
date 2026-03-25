use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    model: Llama,
    // adds memory by storing the session in the structure. each new message is added to the same existing conversation
    chat_session: Chat<Llama>,
}

impl ChatbotV2 {
    #[allow(dead_code)]
    // creates chat session once chatbot is first created
    // DIFF BETWEEN V1 and V2: 
    // V2: we create the session in new so it can be reused later
    // V1: session created inside chat_with_user to it was recreated everytime
    pub fn new(model: Llama) -> ChatbotV2 {
        let chat_session = model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        return ChatbotV2 {
            // Whatever you decide to store in the struct
            // you need to make sure you pass here!
            model: model,
            chat_session: chat_session,
        };
    }

    #[allow(dead_code)]
    // adds new message directly to self.chat_session, which already contains earlier conversation history, so response depends on past messages
    pub async fn chat_with_user(&mut self, message: String) -> String {
        // new message is being added to the existing saved session
        let output = self.chat_session.add_message(message).await;

        match output {
            // If the model successfully returns a response, return it
            Ok(response) => response,

            // If something goes wrong, return a fallback error message
            Err(_) => String::from("Sorry, I could not generate a response."),
        }
    }
}
