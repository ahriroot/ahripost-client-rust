#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

fn main()-> Result<(), Box<dyn std::error::Error>> {
    let mut app = tauri::Builder::default();

    app = app.invoke_handler(tauri::generate_handler![
        close_splashscreen,
    ]);
    app.run(tauri::generate_context!())
        .expect("error while running ahridbms application");
    Ok(())
}
