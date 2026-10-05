use goxlr_model::{AppSnapshot, ChannelName, FaderName};
use goxlr_service::AppService;
use serde::Serialize;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_autostart::ManagerExt;

pub struct DesktopState {
    pub service: Arc<AppService>,
    quitting: AtomicBool,
}

impl DesktopState {
    pub fn new(service: Arc<AppService>) -> Self {
        Self {
            service,
            quitting: AtomicBool::new(false),
        }
    }

    pub fn is_quitting(&self) -> bool {
        self.quitting.load(Ordering::SeqCst)
    }

    pub fn request_quit(&self) {
        self.quitting.store(true, Ordering::SeqCst);
        self.service.request_shutdown();
    }
}

#[tauri::command]
pub async fn get_snapshot(state: State<'_, DesktopState>) -> Result<AppSnapshot, CommandError> {
    Ok(state.service.snapshot().await)
}

#[tauri::command]
pub async fn set_mock_device_enabled(
    enabled: bool,
    state: State<'_, DesktopState>,
) -> Result<AppSnapshot, CommandError> {
    state
        .service
        .set_mock_device_enabled(enabled)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn set_start_minimized(
    enabled: bool,
    state: State<'_, DesktopState>,
) -> Result<AppSnapshot, CommandError> {
    state
        .service
        .set_start_minimized(enabled)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn set_launch_at_startup_enabled(
    enabled: bool,
    app: AppHandle,
    state: State<'_, DesktopState>,
) -> Result<AppSnapshot, CommandError> {
    let autostart = app.autolaunch();
    if enabled {
        autostart
            .enable()
            .map_err(|error| CommandError::new(error.to_string()))?;
    } else {
        autostart
            .disable()
            .map_err(|error| CommandError::new(error.to_string()))?;
    }

    state
        .service
        .set_launch_at_startup(enabled)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn set_selected_device(
    device_id: Option<String>,
    state: State<'_, DesktopState>,
) -> Result<AppSnapshot, CommandError> {
    state
        .service
        .set_selected_device(device_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn set_fader_volume(
    device_id: String,
    session_generation: u64,
    fader: FaderName,
    percent: u8,
    state: State<'_, DesktopState>,
) -> Result<AppSnapshot, CommandError> {
    state
        .service
        .set_fader_volume(device_id, session_generation, fader, percent)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn set_fader_mute(
    device_id: String,
    session_generation: u64,
    fader: FaderName,
    muted: bool,
    state: State<'_, DesktopState>,
) -> Result<AppSnapshot, CommandError> {
    state
        .service
        .set_fader_mute(device_id, session_generation, fader, muted)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn set_fader_assignment(
    device_id: String,
    session_generation: u64,
    fader: FaderName,
    channel: ChannelName,
    state: State<'_, DesktopState>,
) -> Result<AppSnapshot, CommandError> {
    state
        .service
        .set_fader_assignment(device_id, session_generation, fader, channel)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn quit_application(app: AppHandle) -> Result<(), CommandError> {
    quit(app).await;
    Ok(())
}

pub async fn quit(app: AppHandle) {
    let service = app.try_state::<DesktopState>().map(|state| {
        state.request_quit();
        state.service.clone()
    });

    if let Some(service) = service {
        service.shutdown().await;
    }

    app.exit(0);
}

#[derive(Debug, Serialize)]
pub struct CommandError {
    message: String,
}

impl CommandError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<goxlr_service::ServiceError> for CommandError {
    fn from(value: goxlr_service::ServiceError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
