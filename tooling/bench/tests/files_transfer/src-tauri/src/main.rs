// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn app_should_close(exit_code: i32) {
  std::process::exit(exit_code);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![app_should_close])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
