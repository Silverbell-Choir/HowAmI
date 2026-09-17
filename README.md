# HowAmI

> **One click. One report. Everything your computer can expose.**  
> **한 번 실행. 하나의 리포트. 컴퓨터가 제공할 수 있는 시스템 정보를 최대한 한곳에.**

HowAmI는 Windows, macOS, Linux에서 하드웨어·펌웨어·드라이버·운영체제 정보를 **로컬에서만** 수집해 TXT와 JSON 리포트로 저장하는 오픈소스 도구입니다.

HowAmI is an open-source Windows, macOS, and Linux utility that collects hardware, firmware, driver, and operating-system information **locally** and writes TXT and JSON reports.

> **Development status / 개발 상태:** `v0.1.0` build candidate. Windows x64 was built and run on Windows x64; Linux x64/arm64 static binaries were cross-built but not run on target hardware. Windows arm64 and macOS binaries are not included. Keep the repository private until the remaining real-device and privacy checks are complete. / `v0.1.0` 빌드 후보입니다. Windows x64는 Windows x64에서 빌드·실행했고, Linux x64/arm64 정적 바이너리는 크로스빌드했지만 대상 실장비에서 실행하지 않았습니다. Windows arm64와 macOS 바이너리는 포함하지 않았습니다. 남은 실장비·개인정보 검토가 끝날 때까지 Private 유지를 권장합니다.

---

## 한국어

### 핵심 원칙

- 한 번 실행으로 OS와 장치가 노출하는 정보를 최대한 수집합니다.
- 확인할 수 없는 값은 추측하지 않습니다.
- 기본 출력은 사용자의 Desktop에 `HowAmI_Report_*.txt` + `HowAmI_Report_*.json`입니다.
- TXT는 용량·속도·센서값·일부 하드웨어 코드를 사람이 읽기 쉽게 보조 표기합니다.
- JSON은 수집값을 최대한 원형대로 유지하며 `meta.schema_version`으로 형식 버전을 표시합니다.
- 서버 업로드, 원격 분석, 텔레메트리를 구현하지 않습니다.
- 필요한 경우에만 별도 관리자/root child가 수집하고, 일반 실행에서는 최종 리포트를 사용자 프로세스가 작성합니다.

### 현재 수집 범위

#### Windows

- Windows 버전/빌드, 시스템 제조사/모델
- CPU, GPU, GPU 드라이버, 현재 해상도/주사율
- GPU VRAM 보조 조회(Windows 레지스트리가 제공하는 경우)
- 메인보드, BIOS/UEFI
- RAM 모듈/용량/속도/파트넘버/시리얼
- 디스크/물리 디스크/볼륨/펌웨어/상태
- 모니터 및 WMI EDID
- 네트워크, 오디오, USB, Bluetooth, 배터리
- 현재 연결된 PnP 장치
- PnP 드라이버 버전/INF/서명 정보
- TPM / Secure Boot

#### macOS

- macOS 버전, 빌드, 커널
- Hardware / Display & GPU / Memory / Storage / NVMe
- Audio / USB / Network / Bluetooth / Power
- PCI / Thunderbolt & USB4 / Extensions
- System Extensions
- `system_profiler` data type별 독립 수집 및 timeout

#### Linux

- 배포판/커널/CPU
- 메인보드/BIOS/DMI
- RAM 및 `dmidecode` 메모리 모듈 정보(사용 가능한 경우)
- 스토리지/파일시스템/UUID/모델/시리얼/펌웨어 revision
- PCI/USB 장치 및 커널 드라이버/드라이버 버전
- GPU/DRM 카드
- 모니터 연결 상태/모드/EDID 제조사·제품·시리얼
- 네트워크 어댑터
- UEFI/Legacy 부팅 상태, Secure Boot(노출되는 경우), TPM sysfs 정보
- 전원/배터리
- hwmon 온도/팬/전압/전류/전력 등 커널이 노출하는 센서값
- ALSA 오디오, 입력 장치, Bluetooth 컨트롤러
- 설치되어 있는 경우 `lspci`, `lsusb`, `dmidecode` 추가 정보

> 어떤 OS에서도 모든 물리 부품을 100% 식별할 수 있다고 보장하지 않습니다. 일반 PSU처럼 소프트웨어 인터페이스가 없거나 펌웨어/드라이버가 값을 노출하지 않는 장치는 확인할 수 없습니다.

### 권한과 보안

일반 사용자로 실행하면 부모 프로세스는 일반 권한을 유지합니다. 추가 권한이 필요할 때만 별도 관리자/root child가 수집을 수행하고 인증된 임시 handoff를 통해 결과를 돌려준 뒤 종료합니다. 최종 TXT/JSON은 일반 사용자 부모 프로세스가 작성합니다.

관리자/root 환경에서 임의 PATH 검색을 사용하지 않도록 시스템 helper는 신뢰된 경로에서만 실행하며, 수집용 외부 프로세스에는 timeout을 적용합니다.

자세한 내용: [`SECURITY.md`](SECURITY.md)

### 개인정보

상세 리포트에는 다음 값이 포함될 수 있습니다.

- 장치/보드/BIOS/RAM/스토리지/모니터 시리얼
- 시스템 UUID
- MAC 주소
- 호스트명
- PnP/PCI/USB 식별자
- 볼륨/파일시스템 UUID
- 입력 장치 Unique ID
- 드라이버/INF 식별 정보

HowAmI가 이 정보를 네트워크로 전송하지는 않습니다. 외부 공유 전에는 반드시 리포트 내용을 직접 확인하세요.

자세한 내용: [`docs/PRIVACY.md`](docs/PRIVACY.md)

### 사용

```text
HowAmI
HowAmI --output <directory>
HowAmI --no-elevate
```

### 빌드

Rust stable `1.74+`가 필요합니다.

```bash
cargo build --release --locked
```

산출물:

- Windows: `target/release/HowAmI.exe`
- macOS/Linux: `target/release/HowAmI`

현재 준비된 `v0.1.0` 실행 파일:

- [Windows x64](release/dist/v0.1.0/HowAmI-Windows-x64.exe): 네이티브 빌드 및 `--no-elevate` 실행 검증 완료
- [Linux x64](release/dist/v0.1.0/HowAmI-Linux-x64): 정적 musl 크로스빌드, 대상 실장비 실행 미검증
- [Linux arm64](release/dist/v0.1.0/HowAmI-Linux-arm64): 정적 musl 크로스빌드, 대상 실장비 실행 미검증
- [SHA-256 체크섬](release/dist/v0.1.0/SHA256SUMS.txt) 및 [빌드 정보](release/dist/v0.1.0/BUILD-INFO.md)

검증되지 않은 Windows arm64와 macOS 파일은 만들지 않았습니다.

**GitHub Actions/유료 CI는 사용하지 않습니다.** 각 대상 OS/아키텍처에서 로컬 빌드와 실제 장비 검증을 수행합니다.

- 빌드/검증: [`docs/BUILD.md`](docs/BUILD.md)
- 아키텍처: [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- 개인정보: [`docs/PRIVACY.md`](docs/PRIVACY.md)
- 보안: [`SECURITY.md`](SECURITY.md)
- 수동 릴리스: [`docs/RELEASE.md`](docs/RELEASE.md)

---

## English

### Core principles

- Collect as much information as the OS and devices expose in one run.
- Never guess a value that cannot be discovered reliably.
- Write `HowAmI_Report_*.txt` and `HowAmI_Report_*.json` to the user's Desktop by default.
- Add human-readable annotations in TXT for capacities, rates, sensors, and selected hardware codes.
- Preserve discovered values as closely as practical in JSON and expose `meta.schema_version`.
- Do not implement server upload, remote analysis, or telemetry.
- Use a separate Administrator/root child only for collection that benefits from elevation; in the normal flow the user process writes final reports.

### Current collection scope

#### Windows

- Windows version/build and system manufacturer/model
- CPU, GPU, GPU driver, current resolution/refresh rate
- registry-assisted GPU VRAM where Windows exposes it
- mainboard and BIOS/UEFI
- RAM module capacity/speed/part number/serial
- disks, physical disks, volumes, firmware, health/status
- monitors and WMI EDID
- network, audio, USB, Bluetooth, battery
- currently present PnP devices
- PnP driver versions, INF files, signing information
- TPM and Secure Boot

#### macOS

- macOS version/build and kernel
- Hardware / Display & GPU / Memory / Storage / NVMe
- Audio / USB / Network / Bluetooth / Power
- PCI / Thunderbolt & USB4 / Extensions
- System Extensions
- independent `system_profiler` data-type collection with timeouts

#### Linux

- distribution/kernel/CPU
- mainboard/BIOS/DMI
- RAM plus `dmidecode` module detail when available
- storage/filesystem/UUID/model/serial/firmware revision
- PCI/USB devices and kernel driver/version data
- GPU/DRM cards
- monitor connection modes and parsed EDID manufacturer/product/serial
- network adapters
- UEFI/legacy boot state, Secure Boot where exposed, TPM sysfs data
- power supplies and batteries
- kernel-exposed hwmon temperature/fan/voltage/current/power sensors
- ALSA audio, input devices, Bluetooth controllers
- optional `lspci`, `lsusb`, and `dmidecode` detail when installed

> No operating system can guarantee discovery of every physical component. Devices such as conventional PSUs, or values not exposed by firmware/drivers, cannot be identified reliably in software.

### Privileges and security

When started by a normal user, the parent remains unprivileged. A separate Administrator/root child performs elevated collection, returns data through an authenticated temporary handoff, and exits. The normal parent writes the final TXT/JSON files.

System helpers are launched only from trusted locations rather than arbitrary PATH matches while elevated. Collector subprocesses have timeouts.

See [`SECURITY.md`](SECURITY.md).

### Privacy

Detailed reports may include:

- device/board/BIOS/RAM/storage/monitor serial numbers
- system UUID
- MAC addresses
- host name
- PnP/PCI/USB identifiers
- volume/filesystem UUIDs
- input-device unique IDs
- driver/INF identifiers

HowAmI does not transmit these values over the network. Review a report before sharing it externally.

See [`docs/PRIVACY.md`](docs/PRIVACY.md).

### Usage

```text
HowAmI
HowAmI --output <directory>
HowAmI --no-elevate
```

### Build

Stable Rust `1.74+` is required.

```bash
cargo build --release --locked
```

Artifacts:

- Windows: `target/release/HowAmI.exe`
- macOS/Linux: `target/release/HowAmI`

Prepared `v0.1.0` executables:

- [Windows x64](release/dist/v0.1.0/HowAmI-Windows-x64.exe): native build and `--no-elevate` runtime check passed
- [Linux x64](release/dist/v0.1.0/HowAmI-Linux-x64): statically linked musl cross-build; not executed on target hardware
- [Linux arm64](release/dist/v0.1.0/HowAmI-Linux-arm64): statically linked musl cross-build; not executed on target hardware
- [SHA-256 checksums](release/dist/v0.1.0/SHA256SUMS.txt) and [build information](release/dist/v0.1.0/BUILD-INFO.md)

No unverified Windows arm64 or macOS binary was fabricated.

**No GitHub Actions or paid CI are used.** Builds and physical-device validation are performed locally for each target OS/architecture.

- Build/validation: [`docs/BUILD.md`](docs/BUILD.md)
- Architecture: [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- Privacy: [`docs/PRIVACY.md`](docs/PRIVACY.md)
- Security: [`SECURITY.md`](SECURITY.md)
- Manual release: [`docs/RELEASE.md`](docs/RELEASE.md)

## License

MIT License. See [`LICENSE`](LICENSE).
