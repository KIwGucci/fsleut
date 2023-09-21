extern crate globmatch;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

// 検索結果を返す最大数
pub const MAX_RESULT_LEN: usize = 3000;

pub fn item_search(
    rootpath: &str,
    extention: &str,
    searchword: &str,
    deselection: &str,
    is_dir: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let in_extention = if extention.is_empty() { "*" } else { extention };
    // 拡張子
    let pattern = format!("./**/*.{}", in_extention);
    let rootpath = Path::new(rootpath);
    // 拡張子で対象フォルダをglobmatch処理
    let builder = match globmatch::Builder::new(&pattern)
        .case_sensitive(false)
        .build(rootpath)
    {
        Ok(m) => m,
        Err(e) => return Err(e.into()),
    };

    //対象ワードにマッチするか判定しboolを返すクロージャ
    let is_matchword = |x: &PathBuf| {
        let mut jud = false;
        let searchwords = searchword.split_whitespace();

        let path_string: String = if is_dir {
            x.to_string_lossy().to_lowercase()
        } else {
            match x.file_name() {
                Some(filename) => filename.to_string_lossy().to_lowercase(),
                None => "".to_owned(),
            }
        };

        if !deselection.is_empty() && path_string.contains(&deselection.to_lowercase()) {
            // 除外ワード処理
            jud = false;
        } else if searchword.is_empty() {
            // 検索語が空の時は全てtrue
            jud = true
        } else {
            // 検索語を含むか確認
            for wd in searchwords {
                jud = path_string.contains(&wd.to_lowercase());
                if !jud {
                    break;
                }
            }
        }

        jud
    };

    // 検索語でフィルタ処理

    let paths: Vec<PathBuf> = builder.into_iter().flatten().collect();

    let mut result: Vec<String> = vec![];
    for p in paths.iter().filter(|x| is_matchword(x)) {
        // 結果の数が上限を満たすまで処理
        if result.len() > MAX_RESULT_LEN {
            break;
        } else {
            result.push(p.to_string_lossy().to_string());
        }
    }
    Ok(result)
}

pub fn opendir(fpath: &PathBuf, is_file_open: bool) -> Result<(), Box<dyn Error>> {
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
