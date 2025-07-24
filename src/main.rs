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

    if config.output_file.is_some() {
        let path = config.output_file.as_ref().unwrap();
        fs::write(path, result)
            .with_context(|| format!("Failed to write to output file: {}", path.display()))?;
        if config.verbose {
            println!("✅ Successfully generated file: {}", path.display());
        }
    } else {
        println!("{}", result);
    }

    Ok(())
}
