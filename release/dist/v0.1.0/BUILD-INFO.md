# HowAmI v0.1.0 build information / 빌드 정보

- Source commit / 소스 커밋: `d96388c5022f0c80720eb3e5d606d24241456e23`
- Build date / 빌드 날짜: 2026-09-17
- CI: not used / 사용하지 않음
- Rust stable: `rustc 1.98.1`, `cargo 1.98.1`
- Minimum-version check / 최소 버전 확인: `rustc/cargo 1.74.0` Windows x64 test and release build passed

## Included artifacts / 포함 산출물

| File | Target | Size | Build and validation |
| --- | --- | ---: | --- |
| `HowAmI-Windows-x64.exe` | `x86_64-pc-windows-msvc` | 572,416 bytes | Native Windows x64 stable build; fmt, 5 tests, clippy with warnings denied, release build, and `--no-elevate` runtime TXT/JSON structure check passed |
| `HowAmI-Linux-x64` | `x86_64-unknown-linux-musl` | 1,251,704 bytes | `rust-lld` static cross-build; target clippy, test-binary link with `cargo test --no-run`, and release build passed; not executed on Linux hardware |
| `HowAmI-Linux-arm64` | `aarch64-unknown-linux-musl` | 1,254,224 bytes | `rust-lld` static cross-build; target clippy, test-binary link with `cargo test --no-run`, and release build passed; not executed on Linux ARM64 hardware |

The Windows executable is an x86-64 PE32+ console application. The Linux x64 file is a static PIE ELF x86-64 executable, and the Linux arm64 file is a statically linked ELF AArch64 executable. All three files are unsigned.

Windows 실행 파일은 x86-64 PE32+ 콘솔 프로그램입니다. Linux x64는 static PIE ELF x86-64, Linux arm64는 정적 링크 ELF AArch64 실행 파일입니다. 세 파일 모두 서명되지 않았습니다.

## Targets without artifacts / 산출물이 없는 대상

- Windows arm64: target clippy passed, but release linking failed because ARM64 MSVC/CRT libraries were not installed. No executable was produced.
- macOS x64/arm64: target clippy passed, but this Windows environment has no Apple SDK, Apple linker, or macOS hardware. No executables were produced.
- Windows arm64: 대상 clippy는 통과했지만 ARM64 MSVC/CRT 라이브러리가 설치되지 않아 release 링크에 실패했습니다. 실행 파일을 만들지 않았습니다.
- macOS x64/arm64: 대상 clippy는 통과했지만 현재 Windows 환경에는 Apple SDK, Apple 링커, macOS 실장비가 없습니다. 실행 파일을 만들지 않았습니다.

The Linux candidates must be executed on real target hardware before an official Release. Windows arm64 and both macOS targets require native/correctly provisioned build environments and real-device validation.

Linux 후보는 공식 Release 전에 각 대상 실장비에서 실행해야 합니다. Windows arm64와 두 macOS 대상은 적절한 빌드 환경과 실장비 검증이 필요합니다.
