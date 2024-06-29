extern crate globmatch;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

// 検索結果を返す最大数
pub const MAX_RESULT_LEN: usize = 3000;

pub fn item_search(
    rootpath: &str,
    extention: &str,
    searchword: &str,
) -> Result<Vec<String>, String> {
    let in_extention = if extention.is_empty() { "*" } else { extention };
    // 拡張子
    let pattern = format!("./**/*.{}", in_extention);
    let rootpath = Path::new(rootpath);
    // 拡張子で対象フォルダをglobmatch処理
    let builder = globmatch::Builder::new(&pattern)
        .case_sensitive(false)
        .build(rootpath)?;

    // 検索語でフィルタ処理

    let paths = builder
        .into_iter()
        .filter_entry(|p| !globmatch::is_hidden_entry(p))
        .flatten()
        .filter(|p| is_matchword(p, searchword))
        .take(MAX_RESULT_LEN);

    let result: Vec<String> = paths.map(|p| p.to_string_lossy().to_string()).collect();
    Ok(result)
}
fn is_matchword(itempath: &Path, searchword: &str) -> bool {
    if searchword.trim().is_empty() {
        return true;
    }

    let path_string = itempath.to_string_lossy();
    let mut jud = false;
    let searchwords = searchword.split_whitespace();

    // 検索語を全て含むか確認
    for wd in searchwords {
        jud = path_string.contains(&wd.to_lowercase());
        if !jud {
            // wdワードを含んでないときはjudはfalseで即座にbreak
            break;
        }
    }
    jud
}

pub fn opendir(fpath: &PathBuf, is_file_open: bool) -> Result<(), io::Error> {
    // 対象ファイルのフォルダをファインダーで開く
    if is_file_open {
        #[cfg(target_os = "macos")]
        Command::new("open").arg(fpath).spawn()?;

        #[cfg(target_os = "windows")]
        Command::new("explorer")
            .arg(fpath.parent().unwrap())
            .spawn()?;

        #[cfg(target_os = "linux")]
        Command::new("xdg-open")
            .arg(fpath.parent().unwrap())
            .spawn()?;
    } else {
        #[cfg(target_os = "macos")]
        Command::new("open").arg(fpath.parent().unwrap()).spawn()?;

        #[cfg(target_os = "windows")]
        Command::new("explorer")
            .arg(fpath.parent().unwrap())
            .spawn()?;

        #[cfg(target_os = "linux")]
        Command::new("xdg-open")
            .arg(fpath.parent().unwrap())
            .spawn()?;
    }
    Ok(())
}
