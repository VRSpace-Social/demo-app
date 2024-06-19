// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::StreamExt;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State, WindowBuilder, WindowEvent, WindowUrl};
use tokio::sync::oneshot;
use tokio_tungstenite::connect_async_tls_with_config;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::header::{HeaderValue, USER_AGENT};
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

#[derive(Clone)]
struct SharedState {
    window_ready: bool,
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_settings_window(app: AppHandle) -> Result<(), String> {
    let result = WindowBuilder::new(&app, "settings", WindowUrl::App("websocket".into()))
        .fullscreen(false)
        .resizable(true)
        .title("WebSocket")
        .center()
        .build();
    match result {
        Ok(_) => {
            println!("Window Created Successfully!");
            Ok(())
        }
        Err(err) => {
            println!("Failed to Create Window {}", err);
            Err("Failed to create Window".to_string())
        }
    }
}

#[tauri::command]
async fn start_ws(
    app: AppHandle,
    shared_state: State<'_, Arc<Mutex<SharedState>>>,
    authcookie: &str
) -> Result<(), String> {
    let auth_cookie = authcookie.to_string();
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let shutdown_tx = Arc::new(Mutex::new(Some(shutdown_tx)));
    
    {
        let mut state = shared_state.lock().unwrap();
        state.shutdown_tx = shutdown_tx.clone();
    }

    // Listen for the window close event
    let shutdown_tx_clone = shutdown_tx.clone();
    app.get_window("main")
        .unwrap()
        .on_window_event(move |event| {
            if let WindowEvent::CloseRequested { .. } = event {
                // Signal the WebSocket to shut down
                if let Some(tx) = shutdown_tx_clone.lock().unwrap().take() {
                    let _ = tx.send(());
                }
            }
        });

    let app_handle_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        println!("Got signal to WebSocket client...");
        start_websocket_client(app_handle_clone, shutdown_rx, &auth_cookie).await;
    });

    Ok(())
}

#[tauri::command]
async fn stop_ws(shared_state: State<'_, Arc<Mutex<SharedState>>>) -> Result<bool, String> {
    let state = shared_state.lock().unwrap();
    let x = if let Some(tx) = state.shutdown_tx.lock().unwrap().take() {
        let _ = tx.send(());
        Ok(true)
    } else {
        Ok(false)
    }; x
}

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

#[tokio::main]
async fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                            item_handle.set_title("Show").unwrap();
                        } else {
                            window.show().unwrap();
                            item_handle.set_title("Hide").unwrap();
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .setup(|app| {
            let shared_state = Arc::new(Mutex::new(SharedState {
                window_ready: false,
                shutdown_tx: Arc::new(Mutex::new(None)),
            }));

            // Listen for the window created event to set the window as ready
            let shared_state_clone = shared_state.clone();
            app.handle()
                .get_window("main")
                .unwrap()
                .on_window_event(move |event| {
                    if let WindowEvent::Resized { .. } = event {
                        let mut state = shared_state_clone.lock().unwrap();
                        state.window_ready = true;
                    }
                });

            Ok(())
        })
        .manage(Arc::new(Mutex::new(SharedState {
            window_ready: false,
            shutdown_tx: Arc::new(Mutex::new(None)),
        })))
        .invoke_handler(tauri::generate_handler![
            greet,
            open_settings_window,
            start_ws,
            stop_ws
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn start_websocket_client(app_handle: AppHandle, shutdown_rx: oneshot::Receiver<()>, auth_cookie: &str) {
    println!("Preparing to start up WebSocket...");
    let url = Url::parse(
        format!("wss://pipeline.vrchat.cloud/?authToken={}", auth_cookie).as_str()
    )
    .unwrap();

    // Build the custom WebSocket request
    let mut request = url
        .into_client_request()
        .expect("Failed to parse URL as client request");
    request
        .headers_mut()
        .insert(USER_AGENT, HeaderValue::from_static("VRSpace-App@0.0.1"));

    let (ws_stream, _) = connect_async_tls_with_config(request, None, None)
        .await
        .expect("Failed to connect");
    println!("WebSocket connected");
    app_handle.emit_all("ws_connected", "ws_connected").unwrap();

    let app_handle_clone = app_handle.clone();
    tokio::spawn(handle_connection(ws_stream, app_handle_clone, shutdown_rx));
}

async fn handle_connection(
    mut ws_stream: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    app_handle: AppHandle,
    mut shutdown_rx: oneshot::Receiver<()>,
) {
    loop {
        tokio::select! {
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // Parse the text message as JSON
                        match serde_json::from_str::<Value>(&text) {
                            Ok(json) => {
                                // Emit an event to the frontend with the parsed JSON
                                app_handle.emit_all("ws_msg", json.clone()).unwrap();
                            },
                            Err(e) => {
                                eprintln!("Failed to parse JSON: {}", e);
                            }
                        }
                    },
                    Some(Ok(Message::Binary(_bin))) => {
                        println!("Received binary data");
                        // Handle binary data if needed
                    },
                    Some(Ok(_)) => {},
                    Some(Err(e)) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    },
                    None => break,
                }
            },
            _ = &mut shutdown_rx => {
                println!("Received shutdown signal");
                ws_stream.close(None).await.expect("Failed to close WebSocket connection");
                break;
            },
        }
    }
}
