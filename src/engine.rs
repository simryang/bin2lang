use crate::config::Config;
use anyhow::{Context, Result};
use mlua::Lua;
use std::fs;

pub fn run(config: &Config) -> Result<String> {
    // 1. Lua 가상 머신 생성
    let lua = Lua::new();

    // 2. 바이너리 데이터 읽기 및 NULL 처리
    let mut data = fs::read(&config.input_file)
        .with_context(|| format!("Failed to read input file: {}", config.input_file.display()))?;
    if !config.no_null {
        data.push(0x00);
    }

    // 3. Lua 스크립트에 전달할 API 테이블 생성
    let api_table = lua.create_table()?;

    // 3a. 바이트 데이터를 Lua 테이블로 변환
    let data_lua = lua.create_table_from(data.iter().enumerate().map(|(i, &byte)| (i + 1, byte)))?;
    api_table.set("data", data_lua)?;

    // 3b. 나머지 설정 값들을 추가
    api_table.set("array_name", config.array_name.clone())?;
    api_table.set("input_file", config.input_file.to_string_lossy().into_owned())?;
    api_table.set("output_file", config.output_file.as_ref().map(|p| p.to_string_lossy().into_owned()))?;
    api_table.set("array_type", config.array_type.clone())?;
    api_table.set("python_type", config.python_type.clone())?;
    api_table.set("rust_type", config.rust_type.clone())?;
    api_table.set("indent", config.indent)?;

    // 4. API 테이블을 Lua 전역 변수 'BIN2LANG'으로 설정
    lua.globals().set("BIN2LANG", api_table)?;

    // 5. 플러그인 스크립트 파일 읽기 (없으면 C 변환 fallback)
    let plugin_path = format!("plugins/{}.lua", config.lang);
    let plugin_script = fs::read_to_string(&plugin_path);
    if config.lang == "c" && plugin_script.is_err() {
        // Fallback: Rust에서 직접 C array 변환
        let indent = " ".repeat(config.indent);
        let line_length = config.line_length;
        let array_type = &config.array_type;
        let array_name = &config.array_name;
        let mut out = String::new();
        out.push_str(&format!("// bin2lang generated (fallback)\n{} {}[] = {{\n", array_type, array_name));
        for (i, byte) in data.iter().enumerate() {
            if i % line_length == 0 {
                out.push_str(&indent);
            }
            out.push_str(&format!("0x{:02X}", byte));
            if i != data.len() - 1 {
                out.push(',');
            }
            if (i + 1) % line_length == 0 || i == data.len() - 1 {
                out.push_str("\n");
            } else {
                out.push(' ');
            }
        }
        out.push_str("};\n");
        Ok(out)
    } else {
        // 6. 플러그인 실행 및 결과 받기
        let plugin_script = plugin_script
            .with_context(|| format!("Failed to load plugin: '{}'. Make sure it exists in the 'plugins' directory.", plugin_path))?;
        let result: String = lua.load(&plugin_script)
            .set_name(&plugin_path)
            .eval()
            .with_context(|| format!("Error executing plugin '{}'", plugin_path))?;
        Ok(result)
    }
}
