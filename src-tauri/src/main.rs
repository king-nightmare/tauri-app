use actix_web::{web, App, HttpServer, Responder};
use tauri::Manager;

#[tauri::command]
async fn handle_request() -> String {
    // Your backend logic here
    "Response from Rust".into()
}

async fn run_actix() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| async { "Hello from Actix!" }))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

fn main() {
    // Run Actix web server in a separate thread
    std::thread::spawn(|| {
        let sys = actix_rt::System::new();
        sys.block_on(run_actix()).unwrap();
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_request])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
