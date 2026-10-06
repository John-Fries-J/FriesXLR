mod commands;
mod logging;
mod tray;

use commands::{
    get_snapshot, quit_application, set_compressor, set_de_esser, set_equalizer_band,
    set_fader_assignment, set_fader_mute, set_fader_volume, set_launch_at_startup_enabled,
    set_microphone_gain, set_microphone_type, set_mock_device_enabled, set_noise_gate,
    set_routing_route, set_selected_device, set_start_minimized, DesktopState,
};
use goxlr_profile::{AppConfig, ConfigStore};
use goxlr_service::{AppService, ServiceEvent};
use tauri::{AppHandle, Emitter, Manager, WindowEvent};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_window_state::StateFlags;
use tracing::{error, info, warn};

const START_MINIMIZED_ARG: &str = "--minimized";

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            tray::show_main_window(app);
        }))
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED)
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![START_MINIMIZED_ARG]),
        ))
        .setup(|app| {
            let log_guard = logging::init()?;
            app.manage(log_guard);

            let config_store = ConfigStore::default_for_app()
                .map_err(|error| tauri::Error::Anyhow(error.into()))?;
            let initial_config = config_store
                .load_or_default()
                .map_err(|error| tauri::Error::Anyhow(error.into()))?;
            let service = tauri::async_runtime::block_on(AppService::new(config_store))
                .map_err(|error| tauri::Error::Anyhow(error.into()))?;

            spawn_service(service.clone());
            spawn_event_bridge(app.handle().clone(), service.clone());

            app.manage(DesktopState::new(service));
            tray::build(app)?;
            sync_launch_at_startup(app.handle(), initial_config.launch_at_startup);
            apply_initial_window_state(app.handle(), &initial_config);

            info!("FriesXLR desktop started");
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let is_quitting = window
                    .app_handle()
                    .try_state::<DesktopState>()
                    .map(|state| state.is_quitting())
                    .unwrap_or(false);

                if is_quitting {
                    return;
                }

                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_snapshot,
            set_mock_device_enabled,
            set_start_minimized,
            set_launch_at_startup_enabled,
            set_selected_device,
            set_fader_volume,
            set_fader_mute,
            set_fader_assignment,
            set_routing_route,
            set_microphone_type,
            set_microphone_gain,
            set_equalizer_band,
            set_noise_gate,
            set_compressor,
            set_de_esser,
            quit_application
        ])
        .run(tauri::generate_context!())
        .expect("failed to run FriesXLR desktop");
}

fn sync_launch_at_startup(app: &AppHandle, enabled: bool) {
    let autostart = app.autolaunch();
    let result = if enabled {
        autostart.enable()
    } else {
        autostart.disable()
    };

    if let Err(error) = result {
        warn!(%error, enabled, "failed to sync launch-at-startup setting");
    }
}

fn apply_initial_window_state(app: &AppHandle, config: &AppConfig) {
    let start_minimized =
        config.start_minimized || std::env::args().any(|arg| arg == START_MINIMIZED_ARG);

    if start_minimized {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.minimize();
            let _ = window.hide();
        }
    }
}

fn spawn_service(service: std::sync::Arc<AppService>) {
    tauri::async_runtime::spawn(async move {
        service.run().await;
    });
}

fn spawn_event_bridge(app: tauri::AppHandle, service: std::sync::Arc<AppService>) {
    tauri::async_runtime::spawn(async move {
        let mut events = service.subscribe();

        loop {
            match events.recv().await {
                Ok(ServiceEvent::SnapshotChanged(snapshot)) => {
                    if let Err(error) = app.emit("friesxlr://snapshot", snapshot) {
                        error!(%error, "failed to emit snapshot to frontend");
                    }
                }
                Err(error) => {
                    error!(%error, "service event channel failed");
                    break;
                }
            }
        }
    });
}
