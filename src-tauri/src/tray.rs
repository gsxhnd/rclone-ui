use std::sync::Mutex;
use tauri::menu::{Menu, MenuId, MenuItem, PredefinedMenuItem};
use tauri::Wry;
use tauri::{image::Image, tray::TrayIconBuilder, AppHandle};
use tauri::{EventLoopMessage, Manager};

use crate::AppData;

pub enum TrayItem {
    OpenCap,
    TakeScreenshot,
    PreviousRecordings,
    PreviousScreenshots,
    OpenSettings,
    Quit,
}

impl From<TrayItem> for MenuId {
    fn from(value: TrayItem) -> Self {
        match value {
            TrayItem::OpenCap => "open_cap",
            TrayItem::TakeScreenshot => "take_screenshot",
            TrayItem::PreviousRecordings => "previous_recordings",
            TrayItem::PreviousScreenshots => "previous_screenshots",
            TrayItem::OpenSettings => "open_settings",
            TrayItem::Quit => "quit",
        }
        .into()
    }
}

impl TryFrom<MenuId> for TrayItem {
    type Error = String;

    fn try_from(value: MenuId) -> Result<Self, Self::Error> {
        match value.0.as_str() {
            "open_cap" => Ok(TrayItem::OpenCap),
            "take_screenshot" => Ok(TrayItem::TakeScreenshot),
            "previous_recordings" => Ok(TrayItem::PreviousRecordings),
            "previous_screenshots" => Ok(TrayItem::PreviousScreenshots),
            "open_settings" => Ok(TrayItem::OpenSettings),
            "quit" => Ok(TrayItem::Quit),
            value => Err(format!("Invalid tray item id {value}")),
        }
    }
}

pub fn init_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    let state = app.state::<Mutex<AppData>>();
    let a = state.lock().unwrap();
    println!("init_menu {}", a.message);

    let menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, TrayItem::OpenCap, "New Recording", true, None::<&str>)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(
                app,
                TrayItem::PreviousRecordings,
                a.message,
                true,
                None::<&str>,
            )?,
            &MenuItem::with_id(app, "open_settings", "Settings", true, None::<&str>)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(
                app,
                "version",
                format!("Cap v{}", env!("CARGO_PKG_VERSION")),
                false,
                None::<&str>,
            )?,
            &MenuItem::with_id(app, TrayItem::Quit, "Quit", true, None::<&str>)?,
        ],
    );
    drop(a);
    menu
}

pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let menu = init_menu(app)?;
    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(true)
        .build(app)?;

    println!("tray id: {:?}", tray.id());

    let tray_id = tray.id().clone();
    tray.on_menu_event(move |app, event| {
        println!("{}", event.id.as_ref());
        match event.id.as_ref() {
            "open_settings" => {
                let state = app.state::<Mutex<AppData>>();
                let mut a = state.lock().unwrap();
                a.message = "setting";
                match app.tray_by_id(&tray_id) {
                    None => {
                        println!("no open_settings tray icon")
                    }
                    Some(x) => {
                        x.set_title(Some(a.message)).unwrap();
                        let new_menu = init_menu(app).unwrap();
                        // let _ = x.set_menu(Some(new_menu));
                    }
                }
            }
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            _ => todo!(),
        }
    });

    Ok(())
}
