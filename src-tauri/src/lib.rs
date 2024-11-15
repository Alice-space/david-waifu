use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command(async)]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn inference(name: String) -> String {
    // By default, it will connect to localhost:11434
    let ollama = Ollama::default();
    let model = "qwen2.5:72b".to_string();

    // Create a GenerationRequest with the provided model and prompt
    let request = GenerationRequest::new(model, name.to_string());

    // Await the result of the generate method
    match ollama.generate(request).await {
        Ok(res) => res.response,
        Err(e) => format!("Error generating response: {}", e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, inference])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}