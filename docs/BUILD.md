# Build & Run / 빌드 및 실행

## 한국어

### 요구사항

- Rust stable `1.74+`
- `Cargo.lock` 기준 빌드를 위해 `--locked` 사용 권장
- 각 운영체제의 네이티브 빌드 도구

기본 검증 명령:

```bash
cargo fmt --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

보조 스크립트:

Windows:

```powershell
.\scripts\validate-windows.ps1
```

macOS/Linux:

```bash
sh scripts/validate-unix.sh
```

### Windows

```powershell
cargo build --release --locked
.\target\release\HowAmI.exe
```

주요 대상:

```text
x86_64-pc-windows-msvc
aarch64-pc-windows-msvc
```

MSVC 대상은 Visual Studio C++ Build Tools 또는 동등한 MSVC/Windows SDK 환경이 필요합니다.

일반 권한으로 실행하면 필요한 경우 UAC 승인을 요청합니다. 관리자 child는 수집을 수행한 뒤 종료하며, 일반적인 실행 흐름에서 최종 TXT/JSON은 원래 사용자 프로세스가 작성합니다.

### macOS

```bash
cargo build --release --locked
./target/release/HowAmI
```

주요 대상:

```text
x86_64-apple-darwin
aarch64-apple-darwin
```

해당 macOS 대상에 맞는 Apple SDK와 빌드 환경이 필요합니다.

### Linux

```bash
cargo build --release --locked
./target/release/HowAmI
```

주요 대상:

```text
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu
x86_64-unknown-linux-musl
aarch64-unknown-linux-musl
```

일부 상세 정보는 다음 시스템 도구가 설치되어 있을 때 추가됩니다.

```text
lspci
lsusb
dmidecode
```

권한 상승은 신뢰된 시스템 경로의 `pkexec`을 우선 사용하고, 사용할 수 없으면 `sudo`를 사용합니다. 권한 상승을 사용할 수 없거나 사용자가 거부한 경우 일반 권한으로 가능한 정보만 수집합니다.

### 실행 옵션

출력 위치 지정:

```bash
HowAmI --output /path/to/output
```

권한 상승 없이 실행:

```bash
HowAmI --no-elevate
```

### 빌드 산출물

기본 release 빌드 결과:

```text
Windows      target/release/HowAmI.exe
macOS/Linux  target/release/HowAmI
```

배포 파일명은 운영체제와 아키텍처가 명확하게 드러나도록 구성합니다.

```text
HowAmI-Windows-x64.exe
HowAmI-Windows-arm64.exe
HowAmI-macOS-x64
HowAmI-macOS-arm64
HowAmI-Linux-x64
HowAmI-Linux-arm64
```

**빌드된 실행 파일은 Git 소스 트리에 커밋하지 않습니다.** 배포용 바이너리와 `SHA256SUMS.txt`, 선택적인 `BUILD-INFO.md`는 각 버전의 GitHub Release 자산으로 제공합니다.

### 권장 확인 항목

배포 전에는 대상 장비에서 최소한 다음을 확인하는 것이 좋습니다.

1. TXT와 JSON이 정상 생성되는지
2. 주요 하드웨어 이름과 용량/버전이 실제 장비와 일치하는지
3. 관리자/root 승인·거부 모두 정상 동작하는지
4. `--output`과 `--no-elevate`가 정상 동작하는지
5. 장치/API 하나가 실패하거나 timeout되어도 나머지 리포트가 생성되는지
6. 리포트에 포함되는 고유 식별 정보가 예상 범위인지

---

## English

### Requirements

- Stable Rust `1.74+`
- `--locked` is recommended so builds use the committed `Cargo.lock`
- Native build tools for the target operating system

Recommended validation commands:

```bash
cargo fmt --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

Helper scripts:

Windows:

```powershell
.\scripts\validate-windows.ps1
```

macOS/Linux:

```bash
sh scripts/validate-unix.sh
```

### Windows

```powershell
cargo build --release --locked
.\target\release\HowAmI.exe
```

Primary targets:

```text
x86_64-pc-windows-msvc
aarch64-pc-windows-msvc
```

MSVC targets require Visual Studio C++ Build Tools or an equivalent MSVC/Windows SDK environment.

When started as a normal user, HowAmI requests UAC elevation when useful. The Administrator child performs collection and exits; in the normal flow the original user process writes the final TXT/JSON reports.

### macOS

```bash
cargo build --release --locked
./target/release/HowAmI
```

Primary targets:

```text
x86_64-apple-darwin
aarch64-apple-darwin
```

A matching Apple SDK and macOS build environment are required.

### Linux

```bash
cargo build --release --locked
./target/release/HowAmI
```

Primary targets:

```text
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu
x86_64-unknown-linux-musl
aarch64-unknown-linux-musl
```

Additional detail is collected when these system tools are installed:

```text
lspci
lsusb
dmidecode
```

Privilege elevation prefers `pkexec` from trusted system paths and falls back to `sudo`. If elevation is unavailable or declined, HowAmI continues with the information available to the current user.

### Runtime options

Custom output directory:

```bash
HowAmI --output /path/to/output
```

Disable privilege elevation:

```bash
HowAmI --no-elevate
```

### Build artifacts

Default release build output:

```text
Windows      target/release/HowAmI.exe
macOS/Linux  target/release/HowAmI
```

Distribution filenames should make the operating system and architecture explicit.

```text
HowAmI-Windows-x64.exe
HowAmI-Windows-arm64.exe
HowAmI-macOS-x64
HowAmI-macOS-arm64
HowAmI-Linux-x64
HowAmI-Linux-arm64
```

**Compiled binaries are not committed to the Git source tree.** Distribution binaries, `SHA256SUMS.txt`, and an optional `BUILD-INFO.md` are published as assets on the corresponding GitHub Release.

### Recommended checks

Before distribution, verify at least:

1. both TXT and JSON reports are generated,
2. major hardware identities, capacities, and versions match the target machine,
3. Administrator/root approval and denial paths both behave correctly,
4. `--output` and `--no-elevate` work as expected,
5. one failed or timed-out device/API does not prevent the remaining report from being generated,
6. unique identifiers included in the report are within the expected scope.
