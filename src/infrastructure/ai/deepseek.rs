use crate::error::AppError;
use serde::{Deserialize, Serialize};
use worker::Fetch;

const API_URL: &str = "https://api.deepseek.com/v1/chat/completions";
const MODEL: &str = "deepseek-v4-pro";

#[derive(Serialize)]
struct ChatRequest {
    model: &'static str,
    max_tokens: u32,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize)]
struct ChatMessage {
    role: &'static str,
    content: serde_json::Value,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

pub async fn analyze_text(prompt: &str, api_key: &str) -> Result<String, AppError> {
    let body = ChatRequest {
        model: MODEL,
        max_tokens: 2048,
        messages: vec![ChatMessage {
            role: "user",
            content: serde_json::Value::String(prompt.to_string()),
        }],
    };
    call_api(body, api_key).await
}

pub async fn analyze_image(base64_data: &str, media_type: &str, prompt: &str, api_key: &str) -> Result<String, AppError> {
    let data_url = format!("data:{};base64,{}", media_type, base64_data);
    let body = ChatRequest {
        model: MODEL,
        max_tokens: 4096,
        messages: vec![ChatMessage {
            role: "user",
            content: serde_json::json!([
                {"type": "image_url", "image_url": {"url": data_url}},
                {"type": "text", "text": prompt}
            ]),
        }],
    };
    call_api(body, api_key).await
}

async fn call_api(body: ChatRequest, api_key: &str) -> Result<String, AppError> {
    let body_str = serde_json::to_string(&body)?;
    let mut headers = worker::Headers::new();
    headers.set("content-type", "application/json").map_err(|e| AppError::Internal(e.to_string()))?;
    headers.set("authorization", &format!("Bearer {}", api_key)).map_err(|e| AppError::Internal(e.to_string()))?;
    let req = worker::Request::new_with_init(
        API_URL,
        worker::RequestInit::new()
            .with_method(worker::Method::Post)
            .with_body(Some(worker::wasm_bindgen::JsValue::from_str(&body_str)))
            .with_headers(headers),
    )
    .map_err(|e| AppError::Internal(e.to_string()))?;

    let mut resp = Fetch::Request(req.into())
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("DeepSeek API error: {}", e)))?;

    let api_resp: ChatResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("DeepSeek response parse error: {}", e)))?;

    api_resp
        .choices
        .into_iter()
        .next()
        .map(|c| c.message.content)
        .ok_or_else(|| AppError::Internal("empty DeepSeek response".into()))
}
