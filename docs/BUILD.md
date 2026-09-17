# Build & Run / 빌드 및 실행

## 한국어

### 요구사항

- Rust stable toolchain
- 대상 OS에서 직접 빌드하는 것을 기본으로 합니다.
- GitHub Actions/유료 CI는 사용하지 않습니다.

### Windows

```powershell
cargo build --release
.\target\release\HowAmI.exe
```

일반 권한으로 시작하면 HowAmI가 UAC 관리자 권한 상승을 요청합니다. 사용자가 거부하면 제한된 정보만 수집하고 계속할 수 있습니다.

### macOS

```bash
cargo build --release
./target/release/HowAmI
```

필요한 경우 macOS 관리자 인증 대화상자를 통해 권한 상승을 시도합니다.

### Linux

```bash
cargo build --release
./target/release/HowAmI
```

권한 상승은 `pkexec`을 우선 사용하고, 사용할 수 없으면 `sudo`를 시도합니다. 배포판 구성에 따라 자동 권한 상승이 불가능할 수 있습니다.

### 출력 위치 변경

```bash
HowAmI --output /path/to/output
```

권한 상승을 원하지 않을 경우:

```bash
HowAmI --no-elevate
```

---

## English

### Requirements

- Stable Rust toolchain
- Native builds on each target OS are the default workflow.
- GitHub Actions and paid CI are intentionally not used.

### Windows

```powershell
cargo build --release
.\target\release\HowAmI.exe
```

When started without Administrator access, HowAmI requests UAC elevation. If the user declines, it may continue with reduced collection capability.

### macOS

```bash
cargo build --release
./target/release/HowAmI
```

HowAmI attempts to request administrator authorization when necessary.

### Linux

```bash
cargo build --release
./target/release/HowAmI
```

Elevation prefers `pkexec` and falls back to `sudo`. Automatic elevation may not be available on every distribution or desktop environment.

### Custom output directory

```bash
HowAmI --output /path/to/output
```

To explicitly disable privilege elevation:

```bash
HowAmI --no-elevate
```
