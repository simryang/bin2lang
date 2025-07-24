fn format_kst(time: std::time::SystemTime) -> String {
    use chrono::{DateTime, Local};
    let dt: DateTime<Local> = time.into();
    format!("{} KST", dt.format("%Y-%m-%d %H:%M:%S"))
}
use anyhow::{Result, Context};
use clap::Parser;
use std::fs;

const BIN2LANG_VERSION: &str = "0.1.0";

use bin2lang::{run, Config};



fn print_versions(verbose: bool) {
    // main.rs의 mtime을 릴리즈 시간으로 표시
    let main_path = std::env::current_exe().unwrap_or_default();
    let main_mtime = fs::metadata(&main_path)
        .and_then(|meta| meta.modified())
        .map(|mtime| format_kst(mtime))
        .unwrap_or_else(|_| "unknown".to_string());
    println!("bin2lang version: {} (released: {})", BIN2LANG_VERSION, main_mtime);
    let plugin_paths: Vec<_> = match fs::read_dir("plugins") {
        Ok(read_dir) => read_dir.filter_map(|e| e.ok().map(|entry| entry.path())).collect(),
        Err(_) => Vec::new(),
    };
    for path in plugin_paths {
        if path.extension().map(|e| e == "lua").unwrap_or(false) {
            let content = fs::read_to_string(&path).unwrap_or_default();
            let version = content.lines().find_map(|l| {
                if l.contains("-- version:") {
                    Some(l.replace("-- version:", "").trim().to_string())
                } else { None }
            }).unwrap_or("0.1.0".to_string());
            // 플러그인 파일의 mtime
            let mtime = fs::metadata(&path)
                .and_then(|meta| meta.modified())
                .map(|mtime| format_kst(mtime))
                .unwrap_or_else(|_| "unknown".to_string());
            println!("{} version: {} (released: {})", path.file_name().unwrap().to_string_lossy(), version, mtime);
            if verbose {
                println!("{}", path.display());
            }
        }
    }
}

fn main() -> Result<()> {


    // 기존 Config 파싱 및 실행 로직 유지
    let config = Config::parse();

    // verbose 옵션이 켜져 있으면 버전 및 상세 로그 출력
    if config.verbose {
        print_versions(true);
    }

    let result = run(&config).map_err(|e| {
        let plugin_path = format!("plugins/{}.lua", config.lang);
        anyhow::anyhow!("Core engine execution failed.\n  Plugin: {}\n  Options: lang={}, array_type={}, output_file={:?}, indent={}, line_length={}\n  Error: {}",
            plugin_path,
            config.lang,
            config.array_type,
            config.output_file,
            config.indent,
            config.line_length,
            e
        )
    })?;

    if let Some(output_path) = &config.output_file {
        use std::path::{Path, PathBuf};
        let mut path: PathBuf = output_path.clone();
        // output_path가 파일명만 있을 때(경로 구분자 없음, . 또는 ..로 시작하지 않음)
        let is_filename_only = output_path.components().count() == 1
            && !output_path.to_string_lossy().starts_with("./")
            && !output_path.to_string_lossy().starts_with(".\\")
            && !output_path.to_string_lossy().starts_with("..")
            && !output_path.to_string_lossy().contains('/')
            && !output_path.to_string_lossy().contains('\\');
        if is_filename_only {
            // 입력 파일과 동일한 폴더에 저장
            let input_dir = config.input_file.parent().unwrap_or(Path::new("."));
            path = input_dir.join(output_path);
        }
        fs::write(&path, result)
            .with_context(|| format!("Failed to write to output file: {}", path.display()))?;
        if config.verbose {
            let full_path = fs::canonicalize(&path).unwrap_or(path.clone());
            let full_path_str = full_path.to_string_lossy();
            let shown_path = full_path_str.trim_start_matches("\\\\?\\");
            println!("✅ Successfully generated file: {}", shown_path);
        }
    } else {
        println!("{}", result);
    }

    Ok(())
}
