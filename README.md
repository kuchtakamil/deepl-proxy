# DeepL Proxy

A full-stack Rust application that provides a web interface for DeepL translation and text improvement services.

## Project Structure

- `common/` - Shared types between frontend and backend
- `backend/` - Axum web server that proxies requests to DeepL API
- `frontend/` - Yew SPA compiled to WebAssembly

## Prerequisites

1. **Rust** (latest stable version)
2. **Trunk** for building the frontend:
   ```bash
   cargo install trunk
   ```
3. **DeepL API Key** - Get one from [DeepL API](https://www.deepl.com/pro-api)

## Setup

1. Clone and navigate to the project:
   ```bash
   cd deepl_proxy
   ```

2. Set your DeepL API key:
   ```bash
   export DEEPL_API_KEY="your-deepl-api-key-here"
   ```

## Running the Application

### Backend (Terminal 1)
```bash
cargo run --bin backend
```
The backend will start on `http://127.0.0.1:3000`

### Frontend (Terminal 2)
```bash
cd frontend
trunk serve
```
The frontend will start on `http://127.0.0.1:8080`

## Usage

1. Open your browser to `http://127.0.0.1:8080`
2. Use the "Translate Text" section to translate text to English
3. Use the "Improve Text" section to improve text quality using DeepL's editing API

## API Endpoints

- `POST /translate` - Translates text (default target: English)
- `POST /improve` - Improves text quality

## Build for Production

### Backend
```bash
cargo build --release --bin backend
```

### Frontend
```bash
cd frontend
trunk build --release
```

The frontend assets will be in `frontend/dist/`.

## Notes

- The frontend makes CORS requests to the backend, which is configured to allow all origins for development
- DeepL API endpoints used:
  - Translation: `https://api-free.deepl.com/v2/translate`
  - Text improvement: `https://api-free.deepl.com/v2/edit` 