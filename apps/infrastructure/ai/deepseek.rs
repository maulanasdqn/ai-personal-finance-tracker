use crate::error::AppError;
use serde::{Deserialize, Serialize};
use worker::Fetch;

const API_URL: &str = "https://api.deepseek.com/v1/chat/completions";
const MODEL: &str = "deepseek-chat";

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

pub async fn analyze_image(
    base64_data: &str,
    media_type: &str,
    prompt: &str,
    api_key: &str,
) -> Result<String, AppError> {
    let data_url = format!("data:{media_type};base64,{base64_data}");
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_node);
    use super::*;

    #[wasm_bindgen_test]
    fn model_is_deepseek_chat() {
        assert_eq!(MODEL, "deepseek-chat");
    }

    #[wasm_bindgen_test]
    fn api_url_contains_deepseek_com() {
        assert!(API_URL.contains("deepseek.com"));
    }
}

async fn call_api(body: ChatRequest, api_key: &str) -> Result<String, AppError> {
    let body_str = serde_json::to_string(&body)?;
    let mut headers = worker::Headers::new();
    headers
        .set("content-type", "application/json")
        .map_err(|_| AppError::Internal)?;
    headers
        .set("authorization", &format!("Bearer {api_key}"))
        .map_err(|_| AppError::Internal)?;
    let req = worker::Request::new_with_init(
        API_URL,
        worker::RequestInit::new()
            .with_method(worker::Method::Post)
            .with_body(Some(worker::wasm_bindgen::JsValue::from_str(&body_str)))
            .with_headers(headers),
    )
    .map_err(|_| AppError::Internal)?;

    let mut resp = Fetch::Request(req)
        .send()
        .await
        .map_err(|_| AppError::Internal)?;

    let api_resp: ChatResponse = resp.json().await.map_err(|_| AppError::Internal)?;

    api_resp
        .choices
        .into_iter()
        .next()
        .map(|c| c.message.content)
        .ok_or(AppError::Internal)
}
