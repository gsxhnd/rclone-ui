use std::sync::Mutex;
use tauri::Manager;
use tauri::State;

mod tray;

#[derive(Default)]
struct AppData {
    message: &'static str,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            app.manage(Mutex::new(AppData { message: "123" }));
            let a = app.handle().clone();
            tray::create_tray(&a).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command(app_handle: tauri::AppHandle, state: State<'_, Mutex<AppData>>) {
    println!("click my_custom_command");
    // let mut state = state.lock().unwrap();

    match app_handle.tray_by_id("1") {
        None => {
            println!("no open_settings tray icon");
            app_handle.exit(0);
        }
        Some(x) => {
            // let state = state::<Mutex<AppData>>();
            let mut a = state.lock().unwrap();
            a.message = "dsadsa";
            println!("init_menu {}", a.message);
            drop(a);

            let m = tray::init_menu(&app_handle).unwrap();
            x.set_title(Some("123")).unwrap();
            x.set_menu(Some(m));
        }
    }
}
