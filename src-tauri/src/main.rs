// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
mod sleuengine;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(Serialize, Deserialize)]
struct SearchToken {
    root_path: String,
    extention: String,
    search_word: String,
    deselection: String,
    is_dir: bool,
}

#[derive(Serialize, Deserialize)]
struct SearchResultStatus {
    items: Vec<String>,
    message: String,
}

#[tauri::command]
async fn search(token: SearchToken) -> SearchResultStatus {
    match sleuengine::item_search(
        &token.root_path,
        &token.extention,
        &token.search_word,
        &token.deselection,
        token.is_dir,
    ) {
        Ok(rst) => {
            let rstlen = &rst.len();

            let search_message = if *rstlen >= sleuengine::MAX_RESULT_LEN {
                format!("{}件以上見つかりました", sleuengine::MAX_RESULT_LEN)
            } else {
                format!("{}件見つかりました", rstlen)
            };

            SearchResultStatus {
                items: rst,
                message: search_message,
            }
        }
        Err(e) => SearchResultStatus {
            items: vec![],
            message: e.to_string(),
        },
    }
}

#[tauri::command]
fn open_item(inpath: &str, is_file_open: bool) -> String {
    let itempath = PathBuf::from_str(inpath).unwrap();
    match sleuengine::opendir(&itempath, is_file_open) {
        Ok(_) => {
            if is_file_open {
                "ファイルを開きました".to_string()
            } else {
                "フォルダを開きました".to_string()
            }
        }
        Err(e) => e.to_string(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search, open_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
