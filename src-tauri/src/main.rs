#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use container::Container;
use tauri::Manager;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::async_runtime as art;
mod event;
mod container;
mod manager;
const CORE_ID:&'static str = "core";
const BIN_RUDANMAKU:&'static str = "rudanmaku";
const SWS_ID:&'static str = "sws";
const BIN_SWS:&'static str = "static-web-server";


#[tauri::command]
async fn get_sws_port(
    port: tauri::State<'_, Arc<RwLock<u16>>>,
) -> Result<u16, String> {
    let port = {
        port.read().await.clone()
    };
    Ok(port)
}

fn main() {
    let mut task_store_inner = HashMap::<&str, container::Container>::new();
    task_store_inner.insert(CORE_ID, Container::new(BIN_RUDANMAKU));
    task_store_inner.insert(SWS_ID, Container::new(BIN_SWS));
    let task_store_raw = Arc::new(RwLock::new(task_store_inner));

    let port_store = Arc::new(RwLock::new(13000u16));
    tauri::Builder::default()
    .manage(port_store.clone())
    .invoke_handler(tauri::generate_handler![get_sws_port])
    .setup(move |app|{
        use event::*;
        let task_store = task_store_raw.clone();
        let main_window_raw = app.get_window("main").unwrap();
        let main_window = main_window_raw.clone();

        art::spawn(async move {
            main_window.emit(SPLASHSCREEN_OPEN, "ル弾幕シェル").unwrap_or_default();

            // initialize your app here instead of sleeping :)
            main_window.emit(SPLASHSCREEN_UPDATE, "ル弾幕シェル").unwrap_or_default();
            {
                let mut task_store = task_store.write().await;
                let sws_contaner = task_store.get_mut(SWS_ID).unwrap();
                let mut port = {port_store.read().await.clone()}; 
                main_window.emit(SPLASHSCREEN_UPDATE, "正在寻找可用端口").unwrap_or_default();
                port = if portpicker::is_free(port) {
                    port
                } else {
                    portpicker::pick_unused_port().expect("no free port for sws")
                };
                {*port_store.write().await = port};

                main_window.emit(SPLASHSCREEN_UPDATE, "正在启动SWS").unwrap_or_default();
                match sws_contaner.start(&[
                    "--host","127.0.0.1", 
                    "--port", port.to_string().as_str(), 
                    "--root", "./webapp"
                ]) {
                    Ok(stream) => {
                        art::spawn(container::bridge(stream, main_window.clone(), SWS_ID));
                    },
                    Err(err) => {
                        main_window.emit(CONSOLE_OUT, ConsoleOut {
                            tag: ConsoleOutType::Terminated,
                            id: SWS_ID.into(), 
                            output: format!("{:?}", err)
                        }).unwrap_or_default();
                        main_window.emit(SPLASHSCREEN_CLOSE, "启动SWS失败").unwrap_or_default();
                    },
                }
            };
            main_window.emit(SPLASHSCREEN_CLOSE, "一切ok").unwrap_or_default();
        });
        let task_store = task_store_raw.clone();
        let main_window = main_window_raw.clone();
        main_window_raw.listen(START_CORE, move |evt|{
            if let Some(Ok(pld)) = evt.payload().map(|s|serde_json::from_str::<StartCore>(s)) {
                let task_store = task_store.clone();
                let main_window = main_window.clone();
                art::spawn(async move {
                    {
                        let mut task_store = task_store.write().await;
                        let core_contaner = task_store.get_mut(CORE_ID).unwrap();
                        core_contaner.kill();
                        let main_window = main_window.clone();
                        match core_contaner.start(&["config", pld.config_path.as_str()]) {
                            Ok(stream) => {
                                art::spawn(container::bridge(stream, main_window, CORE_ID));
                            },
                            Err(err) => {
                                main_window.emit(CONSOLE_OUT, ConsoleOut {
                                    tag: ConsoleOutType::Terminated,
                                    id: CORE_ID.into(), 
                                    output: format!("{:?}", err)
                                }).unwrap_or_default();
                            },
                        }
                    }
                });
            }
        });
        let main_window = main_window_raw.clone();
        main_window_raw.listen(OPEN_WEBAPP,move |evt|{
            use std::path::{PathBuf};
            if let Some(Ok(pld)) = evt.payload().map(|s|serde_json::from_str::<OpenWebapp>(s)) {
                let mut file = PathBuf::new();
                file.push("webapp-container");
                file.push(format!("index.html?url={}", pld.url));
                let label = format!("webapp::{}", pld.name.clone());
                if let Ok(webapp_window) = tauri::Window::builder(&main_window, label.clone().as_str(), tauri::WindowUrl::App(file))
                .transparent(true)
                .title(pld.name)
                .decorations(false)
                .always_on_top(true)
                .build(){
                    #[cfg(target_os = "windows")]
                    {
                        use windows::Win32::UI::WindowsAndMessaging::*;
                        let hwnd = webapp_window.hwnd().unwrap().0;
                        let hwnd = windows::Win32::Foundation::HWND(hwnd);
                        let unlock_style = unsafe {
                            windows::Win32::UI::WindowsAndMessaging::GetWindowLongA(hwnd, GWL_EXSTYLE)
                        };
                        let lock_style = (WS_EX_APPWINDOW | WS_EX_COMPOSITED | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST).0 as i32;
                        let webapp_window_clone = webapp_window.clone();
                        webapp_window.listen(WEBAPP_LOCK, move |_|{
                            let lock_label = format!("{label}::lock");
                            let mut file = PathBuf::new();
                            file.push("webapp-lock");
                            file.push("index.html");
                            let parent_pos = if let Ok(parent_pos) = webapp_window_clone.inner_position() {
                                parent_pos
                            } else {
                                return;
                            };
                            let parent_width = if let Ok(size) = webapp_window_clone.inner_size(){
                                size.width
                            } else {
                                return;
                            };
                            if let Ok(lock) = tauri::Window::builder(&webapp_window_clone, lock_label.clone().as_str(), tauri::WindowUrl::App(file))
                                .transparent(true)
                                .decorations(false)
                                .title(lock_label.clone())
                                .skip_taskbar(true)
                                
                                // .owner_window(hwnd)
                                .resizable(false)
                                .always_on_top(true)
                                .inner_size(parent_width as f64, 48.0)
                                .position(parent_pos.x as f64, parent_pos.y as f64)
                                // .position(parent_pos.x as f64, parent_pos.x as f64)
                                .build()
                            {

                                let webapp_window_clone_clone = webapp_window_clone.clone();
                                let lock_clone = lock.clone();
                                // webapp_window_clone.set_always_on_top(true).unwrap();
                                lock.listen(WEBAPP_UNLOCK, move |_| {
                                    unsafe {
                                        SetWindowLongA(hwnd, GWL_EXSTYLE, unlock_style);
                                    };
                                    webapp_window_clone_clone.set_skip_taskbar(false).unwrap();
                                    // webapp_window_clone_clone.set_always_on_top(false).unwrap();
                                    webapp_window_clone_clone.emit(WEBAPP_UNLOCK, ()).unwrap_or_default();
                                    lock_clone.close().unwrap();
                                    webapp_window_clone_clone.show().unwrap();
                                });
                                webapp_window_clone.set_always_on_top(true).unwrap();
                                webapp_window_clone.set_skip_taskbar(true).unwrap();
                                unsafe {
                                    SetWindowLongA(hwnd, GWL_EXSTYLE, lock_style);
                                };
                                
                            };
                        });
                        // webapp_window.listen(WEBAPP_UNLOCK, move |_|{
                        //     unsafe {
                        //         use windows::Win32::UI::WindowsAndMessaging::*;
                        //         SetWindowLongA(hwnd, GWL_EXSTYLE, unlock_style);
                        //     };
                        // });
                    }
                } else if let Some(webapp_window) = main_window.get_window(label.clone().as_str()){
                    webapp_window.show().unwrap();
                    webapp_window.center().unwrap();
                }
            }
        });

        Ok(())
    })
    .on_window_event(|evt|{
        match (evt.window().label(), evt.event()) {
            ("main", tauri::WindowEvent::Destroyed) => {
                evt.window().app_handle().exit(0);
            }
            _ => {
                
            }
        }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
    
}
