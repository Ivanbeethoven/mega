use serde::Deserialize;

use crate::AskModel;

pub enum GeminiModels {
    ChatBison001,
    TextBison001,
    EmbeddingGecko001,
    Gemini10ProLatest,
    Gemini10Pro,
    GeminiPro,
    Gemini10Pro001,
    Gemini10ProVisionLatest,
    GeminiProVision,
    Gemini15ProLatest,
    Gemini15Pro001,
    Gemini15Pro,
    Gemini15FlashLatest,
    Gemini15Flash001,
    Gemini15Flash,
    Embedding001,
    TextEmbedding004,
    AQA,
}

impl GeminiModels {
    pub fn as_str(&self) -> &str {
        match self {
            GeminiModels::ChatBison001 => "models/chat-bison-001",
            GeminiModels::TextBison001 => "models/text-bison-001",
            GeminiModels::EmbeddingGecko001 => "models/embedding-gecko-001",
            GeminiModels::Gemini10ProLatest => "models/gemini-1.0-pro-latest",
            GeminiModels::Gemini10Pro => "models/gemini-1.0-pro",
            GeminiModels::GeminiPro => "models/gemini-pro",
            GeminiModels::Gemini10Pro001 => "models/gemini-1.0-pro-001",
            GeminiModels::Gemini10ProVisionLatest => "models/gemini-1.0-pro-vision-latest",
            GeminiModels::GeminiProVision => "models/gemini-pro-vision",
            GeminiModels::Gemini15ProLatest => "models/gemini-1.5-pro-latest",
            GeminiModels::Gemini15Pro001 => "models/gemini-1.5-pro-001",
            GeminiModels::Gemini15Pro => "models/gemini-1.5-pro",
            GeminiModels::Gemini15FlashLatest => "models/gemini-1.5-flash-latest",
            GeminiModels::Gemini15Flash001 => "models/gemini-1.5-flash-001",
            GeminiModels::Gemini15Flash => "models/gemini-1.5-flash",
            GeminiModels::Embedding001 => "models/embedding-001",
            GeminiModels::TextEmbedding004 => "models/text-embedding-004",
            GeminiModels::AQA => "models/aqa",
        }
    }
}

pub struct GeminiClient {
    api_key: String,
    model: GeminiModels,
}

impl GeminiClient {
    pub fn new(api_key: String, model: GeminiModels) -> Self {
        Self { api_key, model }
    }
}

impl AskModel for GeminiClient {
    async fn ask_model(&self, question: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/{}:generateContent?key={}",
            self.model.as_str(),
            self.api_key
        );
        let client = reqwest::Client::new();

        let body = serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": question
                }]
            }]
        })
        .to_string();

        let res = client
            .post(&url)
            .body(body)
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await?;

        let content = res.text().await?;
        let response: GeminiResponse = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse response from Gemini: {}\n{}", content, e))?;
        let content = &response.candidates[0].content;
        if content.role != "model" && content.parts.len() != 1 {
            return Err("Failed to parse response from Gemini".into());
        }
        Ok(match &content.parts[0] {
            Part::Text { text } => text.clone(),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiResponse {
    candidates: Vec<Candidate>,
    // prompt_feedback: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Candidate {
    content: Content,
    // finish_reason: String,
    // index: i32,
    // safetry_ratings: Vec<SafetyRating>,
}

#[derive(Debug, Deserialize)]
struct Content {
    parts: Vec<Part>,
    role: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Part {
    Text { text: String },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct UsageMegadata {
    prompt_token_count: i32,
    candidates_token_count: i32,
    total_token_count: i32,
}

#[cfg(test)]
mod test {
    use crate::AskModel;

    #[tokio::test]
    async fn test_gemini_client() {
        let api_key = super::super::test::get_gemini_key().unwrap();
        let client = super::GeminiClient::new(api_key, super::GeminiModels::Gemini15Flash);
        let res = client.ask_model("who are you").await;
        match res {
            Ok(text) => {
                println!("{}", text);
            }
            Err(e) => {
                println!("{}", e);
                panic!();
            }
        }
    }
}
