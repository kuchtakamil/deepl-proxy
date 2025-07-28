use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslateRequest {
    pub text: String,
    pub target_lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImproveRequest {
    pub text: String,
    pub target_lang: Option<String>,
    pub writing_style: Option<String>,
    pub tone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub result: String,
    pub success: bool,
    pub error: Option<String>,
} 