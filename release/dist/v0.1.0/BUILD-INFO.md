# HowAmI v0.1.0 build information / 빌드 정보

- Source commit / 소스 커밋: `d96388c5022f0c80720eb3e5d606d24241456e23`
- Build date / 빌드 날짜: 2026-09-17
- Rust stable: `rustc 1.98.1`, `cargo 1.98.1`
- Minimum Rust check / 최소 Rust 확인: Windows x64에서 `rustc/cargo 1.74.0` test 및 release build 통과

## Included artifacts / 포함 산출물

| File | Target | Size | Validation / 검증 |
| --- | --- | ---: | --- |
| `HowAmI-Windows-x64.exe` | `x86_64-pc-windows-msvc` | 572,416 bytes | Windows x64 네이티브 build, fmt, 5 tests, clippy, release build, `--no-elevate` 실행 및 TXT/JSON 구조 확인 |
| `HowAmI-Linux-x64` | `x86_64-unknown-linux-musl` | 1,251,704 bytes | `rust-lld` 정적 크로스빌드, target clippy, `cargo test --no-run`, release link 통과. Linux 실장비 실행은 별도 확인 필요 |
| `HowAmI-Linux-arm64` | `aarch64-unknown-linux-musl` | 1,254,224 bytes | `rust-lld` 정적 크로스빌드, target clippy, `cargo test --no-run`, release link 통과. Linux ARM64 실장비 실행은 별도 확인 필요 |

## Binary format / 바이너리 형식

- `HowAmI-Windows-x64.exe`: x86-64 PE32+ console application
- `HowAmI-Linux-x64`: static PIE ELF x86-64
- `HowAmI-Linux-arm64`: statically linked ELF AArch64

현재 세 바이너리는 코드 서명되지 않았습니다.

The three binaries are currently unsigned.

## Not included / 미포함 대상

- Windows ARM64: ARM64 MSVC/CRT 링크 환경이 없어 실행 파일을 생성하지 않음
- macOS x64/ARM64: Apple SDK 및 macOS 빌드 환경이 없어 실행 파일을 생성하지 않음

- Windows ARM64: no executable was produced because an ARM64 MSVC/CRT link environment was unavailable
- macOS x64/ARM64: no executables were produced because an Apple SDK and macOS build environment were unavailable

## Checksums / 체크섬

각 파일의 SHA-256은 같은 디렉터리의 [`SHA256SUMS.txt`](SHA256SUMS.txt)에 기록되어 있습니다.

SHA-256 checksums for all included artifacts are listed in [`SHA256SUMS.txt`](SHA256SUMS.txt).
