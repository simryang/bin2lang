use anyhow::{Result, Context};
use clap::Parser;
use std::fs;

const BIN2LANG_VERSION: &str = "0.1.0";

use bin2lang::{run, Config};



fn print_versions(verbose: bool) {
    println!("bin2lang version: {}", BIN2LANG_VERSION);
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
            println!("{} version: {}", path.file_name().unwrap().to_string_lossy(), version);
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

    let result = run(&config).context("Core engine execution failed.")?;

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
