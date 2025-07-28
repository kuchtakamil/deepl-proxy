use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get_service, post},
    Router,
};
use tower_http::services::fs::ServeDir;
use common::{ApiResponse, ImproveRequest, TranslateRequest};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

#[derive(Clone)]
struct AppState {
    client: reqwest::Client,
    deepl_api_key: String,
}

#[derive(Deserialize)]
struct DeepLTranslateResponse {
    translations: Vec<DeepLTranslation>,
}

#[derive(Deserialize)]
struct DeepLTranslation {
    text: String,
}

#[derive(Serialize)]
struct DeepLTranslateRequest {
    text: Vec<String>,
    target_lang: String,
}

#[derive(Serialize)]
struct DeepLImproveRequest {
    text: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    writing_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tone: Option<String>,
}

#[derive(Deserialize)]
struct DeepLImproveResponse {
    improvements: Vec<DeepLImprovement>,
}

#[derive(Deserialize)]
struct DeepLImprovement {
    detected_source_language: String,
    text: String,
}

async fn translate_text(
    State(state): State<AppState>,
    Json(req): Json<TranslateRequest>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let target_lang = req.target_lang.unwrap_or_else(|| "EN".to_string());
    
    let deepl_req = DeepLTranslateRequest {
        text: vec![req.text],
        target_lang,
    };

    let response = state
        .client
        .post("https://api-free.deepl.com/v2/translate")
        .header("Authorization", format!("DeepL-Auth-Key {}", state.deepl_api_key))
        .json(&deepl_req)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<DeepLTranslateResponse>().await {
                    Ok(deepl_resp) => {
                        if let Some(translation) = deepl_resp.translations.first() {
                            Ok(Json(ApiResponse {
                                result: translation.text.clone(),
                                success: true,
                                error: None,
                            }))
                        } else {
                            Ok(Json(ApiResponse {
                                result: String::new(),
                                success: false,
                                error: Some("No translation returned".to_string()),
                            }))
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse DeepL response: {}", e);
                        Ok(Json(ApiResponse {
                            result: String::new(),
                            success: false,
                            error: Some("Failed to parse response".to_string()),
                        }))
                    }
                }
            } else {
                let status = resp.status();
                let error_text = resp.text().await.unwrap_or_default();
                warn!("DeepL API error: {} - {}", status, error_text);
                Ok(Json(ApiResponse {
                    result: String::new(),
                    success: false,
                    error: Some(format!("API error: {}", status)),
                }))
            }
        }
        Err(e) => {
            warn!("Request failed: {}", e);
            Ok(Json(ApiResponse {
                result: String::new(),
                success: false,
                error: Some("Network error".to_string()),
            }))
        }
    }
}

async fn improve_text(
    State(state): State<AppState>,
    Json(req): Json<ImproveRequest>,
) -> Result<Json<ApiResponse>, StatusCode> {
    let deepl_req = DeepLImproveRequest {
        text: vec![req.text],
        target_lang: req.target_lang,
        writing_style: req.writing_style,
        tone: req.tone,
    };

    let response = state
        .client
        .post("https://api-free.deepl.com/v2/write/rephrase")
        .header("Authorization", format!("DeepL-Auth-Key {}", state.deepl_api_key))
        .json(&deepl_req)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<DeepLImproveResponse>().await {
                    Ok(deepl_resp) => {
                        if let Some(improvement) = deepl_resp.improvements.first() {
                            Ok(Json(ApiResponse {
                                result: improvement.text.clone(),
                                success: true,
                                error: None,
                            }))
                        } else {
                            Ok(Json(ApiResponse {
                                result: String::new(),
                                success: false,
                                error: Some("No improvement returned".to_string()),
                            }))
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse DeepL response: {}", e);
                        Ok(Json(ApiResponse {
                            result: String::new(),
                            success: false,
                            error: Some("Failed to parse response".to_string()),
                        }))
                    }
                }
            } else {
                let status = resp.status();
                let error_text = resp.text().await.unwrap_or_default();
                warn!("DeepL API error: {} - {}", status, error_text);
                Ok(Json(ApiResponse {
                    result: String::new(),
                    success: false,
                    error: Some(format!("API error: {}", status)),
                }))
            }
        }
        Err(e) => {
            warn!("Request failed: {}", e);
            Ok(Json(ApiResponse {
                result: String::new(),
                success: false,
                error: Some("Network error".to_string()),
            }))
        }
    }
}

async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "service": "deepl-proxy",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file if it exists
    dotenvy::dotenv().ok();
    
    tracing_subscriber::fmt::init();

    let deepl_api_key = std::env::var("DEEPL_API_KEY")
        .unwrap_or_else(|_| "your-deepl-api-key-here".to_string());

    if deepl_api_key == "your-deepl-api-key-here" {
        warn!("Using placeholder API key. Set DEEPL_API_KEY environment variable.");
    }

    let state = AppState {
        client: reqwest::Client::new(),
        deepl_api_key,
    };

    // Serve static files from the frontend dist directory
    let static_files_path = std::env::var("STATIC_FILES_PATH").unwrap_or_else(|_| "./frontend/dist".to_string());
    let serve_dir = ServeDir::new(static_files_path);

    let app = Router::new()
        .route("/translate", post(translate_text))
        .route("/improve", post(improve_text))
        .route("/health", axum::routing::get(health_check))
        .fallback_service(get_service(serve_dir))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let bind_addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", bind_addr, port);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Backend server running on http://{}", addr);
    
    axum::serve(listener, app).await?;
    Ok(())
} 