use crate::openai::models::{GPTRequest, GPTResponse, Message};
use reqwest::{blocking::Client, Error};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const OPENAI_API_MODEL: &str = "gpt-4-1106-preview";
const MAX_TOKENS: i32 = 250;

pub fn request_openai(api_key: String, prompt: String) -> Result<String, Error> {
    let client: Client = Client::new();

    let message: Message = Message {
        content: prompt,
        role: "system".to_string(),
    };

    let json_body: GPTRequest = GPTRequest {
        model: OPENAI_API_MODEL.to_string(),
        messages: [message].to_vec(),
        max_tokens: MAX_TOKENS,
    };

    let response: GPTResponse = client
        .post(OPENAI_API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json_body)
        .send()?
        .json()?;

    Ok(response.choices[0].clone().message.content)
}
