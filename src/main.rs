use anyhow::{Result, Context};
use clap::Parser;
use std::fs;

// 라이브러리에서 `run` 함수와 `Config` 구조체를 가져옵니다.
use bin2lang::{run, Config};

fn main() -> Result<()> {
    // 1. 커맨드 라인 인자 파싱
    let config = Config::parse();
    
    // 2. 라이브러리의 `run` 함수에 작업 지시
    let result = run(&config).context("Core engine execution failed.")?;

    // 3. 결과 처리
    if let Some(path) = &config.output_file {
        fs::write(path, result)
            .with_context(|| format!("Failed to write to output file: {}", path.display()))?;
        println!("✅ Successfully generated file: {}", path.display());
    } else {
        println!("{}", result);
    }

    Ok(())
}
