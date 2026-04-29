use tauri::{LogicalSize, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .expect("missing 'main' window");

            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{
                    apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState,
                };
                apply_vibrancy(
                    &window,
                    NSVisualEffectMaterial::HeaderView,
                    Some(NSVisualEffectState::Active),
                    None,
                )
                .expect("apply_vibrancy failed");

                // Keep in sync with `--toolbar-height` in src/styles.css.
                const TOOLBAR_HEIGHT: f64 = 52.0;
                window
                    .set_min_size(Some(LogicalSize::new(480.0, TOOLBAR_HEIGHT + 200.0)))
                    .expect("failed to set min size");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
