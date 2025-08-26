use tauri::menu::{Menu, MenuId, MenuItem, PredefinedMenuItem};
use tauri::{image::Image, tray::TrayIconBuilder, AppHandle};

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

pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, TrayItem::OpenCap, "New Recording", true, None::<&str>)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(
                app,
                TrayItem::PreviousRecordings,
                "Previous Recordings",
                true,
                None::<&str>,
            )?,
            &MenuItem::with_id(app, TrayItem::OpenSettings, "Settings", true, None::<&str>)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(
                app,
                "version",
                format!("Cap v{}", env!("CARGO_PKG_VERSION")),
                false,
                None::<&str>,
            )?,
            &MenuItem::with_id(app, TrayItem::Quit, "Quit Cap", true, None::<&str>)?,
        ],
    )?;
    let _ = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(true)
        .build(app)?;
    Ok(())
}
