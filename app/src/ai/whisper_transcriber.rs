// Octomus — direct OpenAI Whisper API transcription.
// Replaces ServerVoiceTranscriber (deleted with app/src/server/).

use std::time::Duration;

use reqwest::Client;

/// Error type for Whisper transcription.
#[derive(Debug, Clone)]
pub enum WhisperTranscribeError {
    NoApiKey,
    RequestFailed(String),
    ApiError(String),
}

impl std::fmt::Display for WhisperTranscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoApiKey => write!(f, "Whisper API key not configured"),
            Self::RequestFailed(msg) => write!(f, "Network error: {msg}"),
            Self::ApiError(msg) => write!(f, "Whisper API: {msg}"),
        }
    }
}

/// Minimal OpenAI Whisper response.
#[derive(serde::Deserialize)]
struct WhisperResponse {
    text: String,
}

/// Direct OpenAI Whisper API transcriber. BYOK — no cloud involved.
pub struct WhisperTranscriber {
    api_key: String,
    model: String,
    client: Client,
}

impl WhisperTranscriber {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "whisper-1".to_string(),
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .expect("reqwest client build"),
        }
    }

    /// Transcribe base64-encoded WAV audio to text via OpenAI Whisper API.
    pub async fn transcribe_wav(&self, wav_base64: &str) -> Result<String, WhisperTranscribeError> {
        if self.api_key.is_empty() {
            return Err(WhisperTranscribeError::NoApiKey);
        }

        let audio_bytes = base64_decode(wav_base64)
            .ok_or_else(|| WhisperTranscribeError::RequestFailed("invalid base64 audio".into()))?;

        let form = reqwest::multipart::Form::new()
            .text("model", self.model.clone())
            .part(
                "file",
                reqwest::multipart::Part::bytes(audio_bytes)
                    .file_name("audio.wav")
                    .mime_str("audio/wav")
                    .map_err(|e| WhisperTranscribeError::RequestFailed(e.to_string()))?,
            );

        let response = self
            .client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await
            .map_err(|e| WhisperTranscribeError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(WhisperTranscribeError::ApiError(format!("{status}: {body}")));
        }

        let whisper_response: WhisperResponse = response
            .json()
            .await
            .map_err(|e| WhisperTranscribeError::ApiError(e.to_string()))?;

        Ok(whisper_response.text)
    }
}

fn base64_decode(input: &str) -> Option<Vec<u8>> {
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD.decode(input).ok()
}
