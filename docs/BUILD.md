# Build & Run / 빌드 및 실행

## 한국어

### 요구사항

- Rust stable `1.74+` toolchain (`rustup` 권장)
- 대상 OS에서 직접 빌드하는 방식을 기본으로 합니다.
- GitHub Actions/유료 CI는 사용하지 않습니다.

### 로컬 검증 순서

코드 변경 후 대상 OS에서 다음을 순서대로 실행합니다.

```bash
cargo fmt --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

동일 절차를 실행하는 보조 스크립트도 제공합니다.

Windows:

```powershell
.\scripts\validate-windows.ps1
```

macOS/Linux:

```bash
sh scripts/validate-unix.sh
```

`cargo build` 또는 `cargo generate-lockfile`을 처음 실행하면 `Cargo.lock`이 생성됩니다. HowAmI는 애플리케이션이므로 실제 릴리스 빌드를 확정할 때 생성된 `Cargo.lock`도 저장소에 커밋합니다.

### 현재 검증 상태 (2026-09-17)

| 대상 | 결과 | 현재 환경에서 수행한 검증 |
| --- | --- | --- |
| Windows x64 (`x86_64-pc-windows-msvc`) | 실행 파일 포함 | stable `fmt/test/clippy/build`, Rust 1.74 `test/build`, Windows x64에서 `--no-elevate` 실행 및 TXT/JSON 구조 확인 |
| Windows arm64 (`aarch64-pc-windows-msvc`) | 실행 파일 없음 | clippy 통과. ARM64 MSVC/CRT 라이브러리가 설치되지 않아 링크 실패 |
| macOS x64/arm64 | 실행 파일 없음 | 두 Rust target의 clippy 통과. Apple SDK·링커·macOS 실장비가 없어 링크/실행하지 않음 |
| Linux x64/arm64 musl | 실행 파일 포함 | 대상별 clippy, `cargo test --no-run`, 정적 release 크로스빌드 통과. Linux 실장비 실행은 미검증 |

실행 파일과 체크섬은 다음 구조로 저장합니다.

```text
release/dist/v0.1.0/
├── HowAmI-Windows-x64.exe
├── HowAmI-Linux-x64
├── HowAmI-Linux-arm64
├── SHA256SUMS.txt
└── BUILD-INFO.md
```

목록에 없는 대상의 빈 파일이나 추정 산출물은 만들지 않습니다. 현재 산출물의 정확한 빌드 조건과 남은 한계는 `BUILD-INFO.md`에 기록합니다.

### Windows

```powershell
cargo build --release --locked
.\target\release\HowAmI.exe
```

일반 권한으로 실행하면 UAC 승인을 요청합니다. 부모 프로세스는 일반 사용자 권한을 유지하고, 관리자 child는 시스템 정보 수집만 수행한 뒤 종료합니다. 최종 TXT/JSON은 일반 사용자 프로세스가 작성합니다.

기본 대상:

```text
x86_64-pc-windows-msvc
aarch64-pc-windows-msvc
```

각 아키텍처는 해당 환경에서 실제 실행 검증 후 배포합니다.

### macOS

```bash
cargo build --release --locked
./target/release/HowAmI
```

필요한 경우 macOS 관리자 인증 대화상자를 띄워 elevated child를 실행합니다. 최종 리포트는 원래 사용자 프로세스가 저장합니다.

기본 대상:

```text
x86_64-apple-darwin
aarch64-apple-darwin
```

### Linux

```bash
cargo build --release --locked
./target/release/HowAmI
```

권한 상승은 신뢰된 시스템 경로의 `pkexec`을 우선 사용하고, 없으면 `sudo`를 사용합니다. 배포판/데스크톱 환경에 따라 자동 권한 상승이 불가능할 수 있으며, 이 경우 일반 권한으로 수집을 계속하고 경고를 리포트에 기록합니다.

기본 대상:

```text
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu
x86_64-unknown-linux-musl
aarch64-unknown-linux-musl
```

현재 저장된 Linux 배포 후보는 Windows x64에서 `rust-lld`로 링크한 정적 musl 바이너리입니다. 크로스빌드와 테스트 바이너리 링크는 통과했지만 Linux 실장비에서 실행하기 전까지 공식 검증 완료로 간주하지 않습니다.

일부 Linux 상세 정보는 다음 도구가 설치되어 있을 때 추가됩니다.

```text
lspci
lsusb
dmidecode
```

### 출력 위치 변경

```bash
HowAmI --output /path/to/output
```

권한 상승을 원하지 않을 경우:

```bash
HowAmI --no-elevate
```

### 실장비 검증

Public 전환이나 Release 전에 최소한 다음을 확인합니다.

1. TXT와 JSON이 모두 생성되는지
2. CPU/GPU/메인보드/BIOS/RAM/스토리지/모니터 등 주요 장치명이 실제 장비와 일치하는지
3. 드라이버/펌웨어 버전이 OS가 제공하는 값과 일치하는지
4. 관리자/root 승인과 거부 모두 정상 동작하는지
5. `--output` 경로가 권한 상승 후에도 유지되는지
6. 일반 사용자 실행 시 최종 리포트가 macOS/Linux에서 root 소유로 생성되지 않는지
7. 예상치 못한 개인정보/고유 식별 정보가 포함되지 않는지
8. 장치/API 하나가 실패하거나 timeout되어도 나머지 리포트가 생성되는지
9. TXT의 보조 단위 표기가 원시 JSON 값과 모순되지 않는지

---

## English

### Requirements

- Stable Rust `1.74+` toolchain (`rustup` recommended)
- Native builds on each target OS are the default workflow.
- GitHub Actions and paid CI are intentionally not used.

### Local validation sequence

Run the following on each target OS after code changes:

```bash
cargo fmt --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

Helper scripts run the same sequence.

Windows:

```powershell
.\scripts\validate-windows.ps1
```

macOS/Linux:

```bash
sh scripts/validate-unix.sh
```

The first `cargo build` or `cargo generate-lockfile` creates `Cargo.lock`. HowAmI is an application, so commit the lockfile produced by the validated release toolchain before the first Release.

### Current validation status (2026-09-17)

| Target | Result | Validation performed in the current environment |
| --- | --- | --- |
| Windows x64 (`x86_64-pc-windows-msvc`) | Executable included | stable `fmt/test/clippy/build`, Rust 1.74 `test/build`, and a Windows x64 `--no-elevate` run with TXT/JSON structure checks |
| Windows arm64 (`aarch64-pc-windows-msvc`) | No executable | clippy passed; linking failed because the ARM64 MSVC/CRT libraries are not installed |
| macOS x64/arm64 | No executables | clippy passed for both Rust targets; linking/running was not attempted without an Apple SDK, linker, or macOS hardware |
| Linux x64/arm64 musl | Executables included | target clippy, `cargo test --no-run`, and static release cross-builds passed; not executed on Linux hardware |

Executables and checksums use this layout:

```text
release/dist/v0.1.0/
├── HowAmI-Windows-x64.exe
├── HowAmI-Linux-x64
├── HowAmI-Linux-arm64
├── SHA256SUMS.txt
└── BUILD-INFO.md
```

Do not create empty or guessed artifacts for targets not listed. `BUILD-INFO.md` records the exact build conditions and remaining limitations.

### Windows

```powershell
cargo build --release --locked
.\target\release\HowAmI.exe
```

When started as a normal user, HowAmI requests UAC approval. The parent remains unprivileged; an Administrator child performs collection only and exits. The normal user process writes the final TXT/JSON files.

Primary targets:

```text
x86_64-pc-windows-msvc
aarch64-pc-windows-msvc
```

Validate each architecture on real hardware before distributing it.

### macOS

```bash
cargo build --release --locked
./target/release/HowAmI
```

HowAmI requests administrator authorization for an elevated collection child when needed. Final reports are written by the original user process.

Primary targets:

```text
x86_64-apple-darwin
aarch64-apple-darwin
```

### Linux

```bash
cargo build --release --locked
./target/release/HowAmI
```

Elevation prefers `pkexec` from trusted system paths and falls back to `sudo` only when `pkexec` is unavailable. If automatic elevation cannot be performed, HowAmI continues with standard-user collection and records a warning.

Primary targets:

```text
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu
x86_64-unknown-linux-musl
aarch64-unknown-linux-musl
```

The currently stored Linux candidates are static musl binaries linked with `rust-lld` on Windows x64. Cross-compilation and test-binary linking passed, but they are not considered fully validated until executed on real Linux hardware.

Additional Linux detail is collected when these tools are installed:

```text
lspci
lsusb
dmidecode
```

### Custom output directory

```bash
HowAmI --output /path/to/output
```

Disable privilege elevation:

```bash
HowAmI --no-elevate
```

### Physical-device validation

Before making the repository public or publishing a Release, verify at least:

1. both TXT and JSON reports are created,
2. CPU/GPU/mainboard/BIOS/RAM/storage/monitor identities match the hardware,
3. driver/firmware versions match values exposed by the OS,
4. both approval and denial of Administrator/root access behave correctly,
5. `--output` survives the elevation flow,
6. in the normal user-started flow, final reports are not root-owned on macOS/Linux,
7. no unexpected personal or unique identifiers appear,
8. one failed/timed-out device or API does not prevent the remaining report from being produced,
9. human-readable TXT annotations remain consistent with the raw JSON values.
