# bin2lang

## 프로젝트 개요

**bin2lang**는 바이너리 파일을 다양한 프로그래밍 언어의 소스 코드 배열로 변환하는 CLI 도구입니다. Lua 플러그인 기반으로 설계되어, C/Python/Rust 등 여러 언어로 확장 가능합니다. 향후 WASM, 바이너리 템플릿 등 다양한 엔진 추가도 고려된 구조입니다.

## 주요 기능
## 프로젝트 소개
**bin2lang**는 바이너리 파일을 다양한 프로그래밍 언어의 소스 코드 배열로 변환하는 CLI 도구입니다. 누구나 손쉽게 바이너리 데이터를 C, Python, Rust 등 다양한 언어 코드로 변환할 수 있습니다.

## 빌드 및 설치 방법
```bash
# 1. 저장소 클론

## 폴더 구조
```
# 2. 릴리즈 모드로 빌드
bin2lang/
```

## 사용법
```bash
# C언어 코드로 변환하기
├── src/
│   ├── main.rs        # CLI 진입점
# Python 바이트 코드로 변환하기
│   ├── lib.rs         # 라이브러리 API
```

## 플러그인 추가 방법
- `plugins` 폴더에 새로운 언어의 Lua 파일(예: `go.lua`)을 추가하면 바로 새 언어 지원이 가능합니다.
- 각 Lua 파일은 해당 언어의 배열 변환 로직을 담당합니다.
│   ├── config.rs      # Config 구조체 및 옵션
│   └── engine.rs      # Lua 엔진 및 변환 로직
├── plugins/
│   ├── c.lua          # C 배열 변환 플러그인
│   ├── python.lua     # Python 배열 변환 플러그인
│   └── rust.lua       # Rust 배열 변환 플러그인
├── Cargo.toml         # Rust 프로젝트 설정
├── .gitignore         # 빌드/IDE/테스트 파일 제외
└── README.md          # 프로젝트 설명
```

## 빌드 방법
1. Rust 및 Cargo 설치
2. 프로젝트 폴더에서 아래 명령 실행
   ```powershell
   cargo build --release
   ```
   빌드 결과는 `target/release/bin2lang.exe`에 생성됩니다.

## 사용법
### 기본 변환
```powershell
cargo run -- -l c "D:\경로\파일명.bin"
```

### 출력 파일 지정
```powershell
cargo run -- -l c "D:\경로\파일명.bin" -o output.c
```

### 배열명 지정
```powershell
cargo run -- -l c "D:\경로\파일명.bin" --array-name my_array
```

### 널 종료 바이트 미포함
```powershell
cargo run -- -l c "D:\경로\파일명.bin" --no-null
```

### 실제 예시
```powershell
cargo run -- -l c "D:\user\Documents\GitHub\bin2lang\Web_page.html" -o output.c
```

## 기여 및 문의
새로운 언어 플러그인이나 엔진 제안은 언제든 환영합니다. 궁금한 점이나 개선 요청은 GitHub Issues를 통해 남겨주세요.
최신 사용법 및 상세 옵션은 `cargo run -- --help`로 확인하세요.

---
기타 참고 사항:
- `.gitignore`에 빌드, IDE, 테스트 관련 파일 제외 규칙 포함
- Rust 2021 edition, 주요 의존성: `anyhow`, `clap`, `mlua`