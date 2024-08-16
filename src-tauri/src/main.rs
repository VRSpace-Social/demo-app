// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use log::{error, info, warn, debug};
use fern::colors::{Color, ColoredLevelConfig};
use chrono::Local;
use std::path::PathBuf;
use futures_util::StreamExt;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State, WindowEvent};
use tokio::sync::oneshot;
use tokio_tungstenite::connect_async_tls_with_config;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::header::{HeaderValue, USER_AGENT};
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;
use reqwest::header::USER_AGENT as REQ_USER_AGENT;
pub use vrchatapi::apis;
use vrchatapi::models::{EitherUserOrTwoFactor, TwoFactorAuthCode};
use reqwest::cookie::{Jar, CookieStore};


use reqwest::Client;

fn setup_logger() -> Result<(), fern::InitError> {
    // Configure colors for log levels
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Magenta);

    // Create log directory if it doesn't exist
    #[cfg(windows)]
    let app_data = std::env::var("LOCALAPPDATA").expect("No APP_DATA directory");

    // Join app_data with the app name and "logs" directory
    #[cfg(windows)]
    let log_dir = PathBuf::from(app_data).join("VRSpaceApp").join("logs");

    std::fs::create_dir_all(&log_dir)?;

    // Generate log file name with current date
    let log_file = log_dir.join(format!("{}.log", Local::now().format("%Y-%m-%d")));

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file)?)
        .apply()?;

    Ok(())
}

#[derive(Clone)]
struct SharedState {
    window_ready: bool,
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
async fn fetch_url(url: String, set_user_agent: bool) -> Result<String, String> {
    let client = reqwest::Client::new();
    let request = client.get(&url);

    let request = if set_user_agent {
        request.header(REQ_USER_AGENT, "VRSpaceApp")
    } else {
        request
    };

    match request.send().await {
        Ok(response) => {
            match response.text().await {
                Ok(body) => {
                    info!("Successfully fetched URL: {}", url);
                    Ok(body)
                },
                Err(e) => {
                    error!("Failed to read response body: {}", e);
                    Err(e.to_string())
                }
            }
        },
        Err(e) => {
            error!("Failed to send request: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn vrc_test(username: &str, password: &str, twofactorcode: Option<String>) -> Result<String, String> {
    let mut config = apis::configuration::Configuration::default();
    config.basic_auth = Some((String::from(username), Some(String::from(password))));
    config.user_agent = Some(String::from("VRSpace-VRCBot/0.1.0 lyzcoote@vrspace.social"));


    // Use our custom client
    config.client = Client::builder().cookie_store(true).build().unwrap();


    let response = apis::authentication_api::get_current_user(&config).await;

    match response {
        Ok(response) => {
            
            match response {
                EitherUserOrTwoFactor::CurrentUser(user) => { 
                    let username = user.display_name;
                    info!("Successfully authenticated user: {}", username);
                    debug!("User ID: {:?}, Username: {:?}", user.id, user.username);

                    // Attempt to fetch online users
                    match apis::system_api::get_current_online_users(&config).await {
                        Ok(online) => {
                            info!("Current Online Users: {}", online);
                            Ok(format!("Authenticated as {}. Current online users: {}", username, online))
                        },
                        Err(e) => {
                            warn!("Authenticated successfully, but failed to fetch online users: {:?}", e);
                            Ok(format!("Authenticated as {}, but couldn't fetch online users", username))
                        }    
                }
            }, EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => {
                info!("Two-factor authentication required");
                if let Some(two_factor_code) = twofactorcode {
                    let two_factor_auth_code = TwoFactorAuthCode { code: two_factor_code };

                    let two_factor_response = vrchatapi::apis::authentication_api::verify2_fa(&config, two_factor_auth_code).await;

                    if let Ok(ref verification_result) = two_factor_response {
                        println!("Verification result: {:?}", verification_result);
                    }

                    match two_factor_response {
                        Ok(verification_result) => {

                        let cookie_jar: Arc<Jar> = config.cookie_jar();

                        println!("Cookies: {:?}", cookie_jar);

                        let url = Url::parse("https://vrchat.com/api/1/auth/twofactorauth/totp/verify").unwrap();
                        if let Some(header_value) = cookie_jar.cookies(&url) {
                            if let Ok(cookies_str) = header_value.to_str() {
                                for cookie in cookies_str.split("; ") {
                                    println!("Cookie: {}", cookie);
                                }
                            } else {
                                println!("Failed to convert HeaderValue to str.");
                            }
                        } else {
                            println!("No cookies found for URL {}", url);
                        }


                            if verification_result.verified {
                                info!("Two-factor authentication successful");
                                Ok("Two-factor authentication successful".to_string())
                            } else {
                                error!("Two-factor authentication failed");
                                Err("Two-factor authentication failed".to_string())
                            }
                        },
                        Err(e) => {
                            error!("Failed to verify two-factor authentication: {:?}", e);
                            Err("Failed to verify two-factor authentication".to_string())
                        }
                    }
                } else {
                    warn!("Two-factor authentication is required");
                    Err("Two-factor authentication is required".to_string())
                }
            }
        }
        }, Err(e) => {
            error!("Failed to authenticate user: {:?}", e);
            Err("Failed to authenticate user".to_string())
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
        // Chiude eventuali connessioni WebSocket esistenti
        if let Some(tx) = state.shutdown_tx.lock().unwrap().take() {
            let _ = tx.send(());
        }
        state.shutdown_tx = shutdown_tx.clone();
    }

    // Ascolta l'evento di chiusura della finestra
    let shutdown_tx_clone = shutdown_tx.clone();
    let window = app.get_window("main").unwrap();
    window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { .. } = event {
            // Segnala la chiusura del WebSocket
            if let Some(tx) = shutdown_tx_clone.lock().unwrap().take() {
                let _ = tx.send(());
            }
        }
    });

    let app_handle_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        info!("Got signal to start WebSocket client...");
        start_websocket_client(app_handle_clone, shutdown_rx, &auth_cookie).await;
    });

    Ok(())
}

#[tauri::command]
async fn stop_ws(shared_state: State<'_, Arc<Mutex<SharedState>>>) -> Result<bool, String> {
    let shutdown_tx = {
        let state = shared_state.lock().unwrap();
        let x = state.shutdown_tx.lock().unwrap().take(); x
    };

    if let Some(tx) = shutdown_tx {
        let _ = tx.send(());
        Ok(true)
    } else {
        Ok(false)
    }
}

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

#[tokio::main]
async fn main() {
    // Initialize logger
    if let Err(e) = setup_logger() {
        eprintln!("Error setting up logger: {}", e);
    }
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

            // Ascolta l'evento di creazione della finestra per impostare lo stato della finestra come pronto
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
            start_ws,
            stop_ws,
            fetch_url,
            vrc_test
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Funzione per avviare il client WebSocket
async fn start_websocket_client(app_handle: AppHandle, shutdown_rx: oneshot::Receiver<()>, auth_cookie: &str) {
    info!("Preparing to start up WebSocket...");
    debug!("Auth cookie: {}", auth_cookie);
    if auth_cookie == "Not Found :(" {
        warn!("No auth cookie found, aborting WebSocket connection");
        app_handle.emit_all("ws_err", "No auth cookie found, aborting WebSocket connection").unwrap();
        return;
    }
    let url = Url::parse(
        format!("wss://pipeline.vrchat.cloud/?authToken={}", auth_cookie).as_str()
    )
    .unwrap();

    info!("Connecting to WebSocket at: {}", url);

    // Costruisce la richiesta WebSocket personalizzata
    let mut request = url
        .into_client_request()
        .expect("Failed to parse URL as client request");
    request
        .headers_mut()
        .insert(USER_AGENT, HeaderValue::from_static("VRSpace-App@0.0.1"));

    let (ws_stream, _) = connect_async_tls_with_config(request, None, false, None)
        .await
        .expect("Failed to connect");
    info!("WebSocket connected");
    app_handle.emit_all("ws_connected", "ws_connected").unwrap();

    let app_handle_clone = app_handle.clone();
    tokio::spawn(handle_connection(ws_stream, app_handle_clone, shutdown_rx));
}

// Funzione per gestire la connessione WebSocket
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
                // Gestisce i messaggi ricevuti dal WebSocket
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // Parsea il messaggio di testo come JSON
                        match serde_json::from_str::<Value>(&text) {
                            
                            Ok(json) => {
                                // Emit an event to the frontend with the parsed JSON
                                //println!("Received JSON: {:?}", json);
                                if json["err"] == "authToken doesn't correspond with an active session" {
                                    error!("Authentication error: {}", json["err"]);
                                    app_handle.emit_all("ws_err", json.clone()).unwrap();
                                    break;
                                }
                                debug!("Received JSON: {}", json);
                                app_handle.emit_all("ws_msg", json.clone()).unwrap();
                            },
                            Err(e) => {
                                error!("Failed to parse JSON: {}", e);
                            }
                        }
                    },
                    Some(Ok(Message::Binary(_bin))) => {
                        debug!("Received binary data");
                    },
                    Some(Ok(_)) => {},
                    Some(Err(e)) => {
                        error!("WebSocket error: {}", e);
                        app_handle.emit_all("ws_err", format!("{}", e)).unwrap();
                        break;
                    },
                    None => break,
                }
            },
            _ = &mut shutdown_rx => {
                // Gestisce il segnale di spegnimento del WebSocket
                info!("Received shutdown signal");
                if let Err(e) = ws_stream.close(None).await {
                    error!("Failed to close WebSocket connection: {}", e);
                }
                app_handle.emit_all("ws_disconnected", "ws_disconnected").unwrap();
                break;
            },
        }
    }
}
