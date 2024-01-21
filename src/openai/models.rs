use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GPTRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: i32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GPTResponse {
    id: String,
    object: String,
    created: i32,
    model: String,
    pub choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Choice {
    index: i32,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub content: String,
    pub role: String,
}
