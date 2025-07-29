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
bin2lang.exe -l c "D:\경로\파일명.bin"
```
또는
```powershell
cargo run -- -l c "D:\경로\파일명.bin"
```
- `-l`, `--lang`: 변환할 언어 (c, python, rust)
- 파일 경로: 변환할 바이너리 파일

※ 참고: `cargo run -- ...`에서 `--`는 cargo 명령어와 실행할 프로그램(bin2lang)의 옵션을 구분하기 위한 구분자입니다. 즉, cargo run의 옵션과 실제 프로그램의 옵션이 혼동되지 않도록, `--` 뒤에 bin2lang의 옵션을 적어줍니다.

### 출력 파일 지정
```powershell
bin2lang.exe -l c "D:\경로\파일명.bin" -o output.c
```
- `-o`, `--output-file`: 결과를 파일로 저장

### 배열명 지정
```powershell
bin2lang.exe -l c "D:\경로\파일명.bin" --array-name my_array
```
- `--array-name`: 배열 변수명 지정

### 널 종료 바이트 미포함
```powershell
bin2lang.exe -l c "D:\경로\파일명.bin" --no-null
```
- `--no-null`: 배열 끝에 널 바이트(0x00) 미포함

### 배열 타입/파이썬 타입/러스트 타입 지정
```powershell
bin2lang.exe -l c "D:\경로\파일명.bin" --array-type "static const unsigned char"
bin2lang.exe -l python "D:\경로\파일명.bin" --python-type "bytes"
bin2lang.exe -l rust "D:\경로\파일명.bin" --rust-type "u8"
```
- `--array-type`: C 배열 타입 지정
- `--python-type`: Python 배열 타입 지정
- `--rust-type`: Rust 배열 타입 지정

### 라인 길이 및 인덴트 지정
```powershell
bin2lang.exe -l c "D:\경로\파일명.bin" --line-length 32 --indent 2
```
- `--line-length`: 한 줄에 출력할 배열 원소 개수
- `--indent`: 들여쓰기(공백) 개수

### 상세 로그 및 버전 정보 출력
```powershell
bin2lang.exe -l c "D:\경로\파일명.bin" -v
bin2lang.exe --version
```
- `-v`, `--verbose`: 상세 로그 및 버전 정보 출력
- `--version`: bin2lang 및 플러그인 버전 정보 출력 후 종료

### 실제 예시
```

- 새로운 언어 지원 시 Lua 파일 추가 및 등록
## 확장성
- 엔진 구조가 모듈화되어 WASM, 바이너리 템플릿 등 다양한 변환 방식 추가 가능
- Lua 외에도 향후 다른 스크립트/엔진 도입 가능

- `.gitignore`에 빌드, IDE, 테스트 관련 파일 제외 규칙 포함
- Rust 2021 edition, 주요 의존성: `anyhow`, `clap`, `mlua`
- 문의: GitHub Issues 활용

---

---

## 변경 내역 (Changelog)

### v0.1.0 (2025-07-25)
- 첫 공식 릴리즈
- CLI 옵션 통합 및 구조 개선 (`Config` 기반)
- --array-type, --python-type, --rust-type, --indent, --line-length 등 세부 옵션 추가
- -v 옵션으로 상세 로그 및 버전 정보 출력 기능 추가
- --version 옵션 및 플러그인별 버전 정보 출력
- Lua 플러그인(c.lua, python.lua, rust.lua) 모두 indent/line_length/type 옵션 지원 및 버전 주석 추가
- 경고 제거(불필요한 import 삭제 등)
- README에 changelog 및 버전 관리 규칙 추가

### 버전 관리 규칙
- 자잘한 수정: 0.0.1 증가
- 하위 기능 추가/변경: 0.1 증가
- 큰 변화/구조 개편: 1.0 증가

최신 사용법 및 상세 옵션은 `cargo run -- --help`로 확인하세요.
