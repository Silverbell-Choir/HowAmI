# HowAmI

> **One click. One report. Everything your computer can expose.**  
> **한 번 실행. 하나의 리포트. 컴퓨터가 제공할 수 있는 시스템 정보를 최대한 한곳에.**

HowAmI는 Windows, macOS, Linux에서 하드웨어·펌웨어·드라이버·운영체제 정보를 **로컬에서 수집**해 사람이 읽기 쉬운 TXT와 구조화된 JSON 리포트로 저장하는 오픈소스 도구입니다.

HowAmI is an open-source Windows, macOS, and Linux utility that **collects system information locally** and writes both human-readable TXT and structured JSON reports.

---

## 한국어

### 주요 기능

- 실행 한 번으로 OS와 장치가 노출하는 시스템 정보를 최대한 수집
- CPU, GPU, 메인보드, BIOS/UEFI, RAM, 스토리지, 모니터, 네트워크, 오디오, USB, Bluetooth 등 확인
- 드라이버·펌웨어·장치 식별 정보 수집
- TXT + JSON 동시 생성
- 확인할 수 없는 값은 추측하지 않음
- 필요한 경우에만 별도 관리자/root 프로세스로 상세 정보 수집
- 서버 업로드, 원격 분석, 텔레메트리 없음

### 지원 플랫폼

#### Windows

- Windows 버전/빌드, 시스템 제조사/모델
- CPU, GPU, GPU 드라이버, 해상도/주사율
- 메인보드, BIOS/UEFI
- RAM 모듈/용량/속도/파트넘버/시리얼
- 디스크/물리 디스크/볼륨/펌웨어/상태
- 모니터 및 WMI EDID
- 네트워크, 오디오, USB, Bluetooth, 배터리
- PnP 장치 및 드라이버 버전/INF/서명 정보
- TPM / Secure Boot

#### macOS

- macOS 버전/빌드, 커널
- Hardware / Display & GPU / Memory / Storage / NVMe
- Audio / USB / Network / Bluetooth / Power
- PCI / Thunderbolt & USB4 / Extensions
- System Extensions

#### Linux

- 배포판/커널/CPU
- 메인보드/BIOS/DMI
- RAM 및 메모리 모듈 정보
- 스토리지/파일시스템/UUID/모델/시리얼/펌웨어
- PCI/USB 장치 및 커널 드라이버
- GPU/DRM 카드
- 모니터 연결 상태/모드/EDID
- 네트워크 어댑터
- UEFI/Legacy, Secure Boot, TPM(노출되는 경우)
- 전원/배터리, hwmon 센서
- ALSA 오디오, 입력 장치, Bluetooth 컨트롤러
- 설치되어 있는 경우 `lspci`, `lsusb`, `dmidecode` 추가 정보

> 어떤 OS에서도 모든 물리 부품을 100% 식별할 수 있는 것은 아닙니다. 일반 PSU처럼 소프트웨어 인터페이스가 없거나 펌웨어/드라이버가 값을 노출하지 않는 장치는 확인할 수 없습니다.

### 실행 결과

기본적으로 Desktop에 다음 두 파일을 생성합니다.

```text
HowAmI_Report_YYYYMMDD_HHMMSS_mmm.txt
HowAmI_Report_YYYYMMDD_HHMMSS_mmm.json
```

TXT는 사람이 읽기 쉬운 형식이며, JSON은 자동 분석이나 다른 도구와의 연동에 사용할 수 있는 구조화 데이터입니다. JSON 형식 버전은 `meta.schema_version`으로 확인할 수 있습니다.

### 사용법

```text
HowAmI
HowAmI --output <directory>
HowAmI --no-elevate
```

- 기본 실행: 필요한 경우 관리자/root 권한을 요청해 상세 정보를 수집합니다.
- `--output`: 출력 디렉터리를 직접 지정합니다.
- `--no-elevate`: 권한 상승 없이 가능한 정보만 수집합니다.

### 개인정보

리포트에는 하드웨어 시리얼, 시스템 UUID, MAC 주소, 호스트명, 장치 ID, 볼륨 UUID 등 **고유 식별 정보가 포함될 수 있습니다.** HowAmI는 수집한 정보를 외부로 전송하지 않습니다.

다른 사람이나 서비스에 리포트를 공유하기 전에는 내용을 직접 확인하세요.

자세한 내용: [`docs/PRIVACY.md`](docs/PRIVACY.md)

### 빌드

Rust stable `1.74+`가 필요합니다.

```bash
cargo build --release --locked
```

산출물:

- Windows: `target/release/HowAmI.exe`
- macOS/Linux: `target/release/HowAmI`

자세한 빌드 방법: [`docs/BUILD.md`](docs/BUILD.md)

### 제공 중인 v0.1.0 바이너리

| 파일 | 대상 | 상태 |
| --- | --- | --- |
| [`HowAmI-Windows-x64.exe`](release/dist/v0.1.0/HowAmI-Windows-x64.exe) | Windows x64 | Windows x64에서 빌드 및 기본 실행 확인 |
| [`HowAmI-Linux-x64`](release/dist/v0.1.0/HowAmI-Linux-x64) | Linux x64 musl | 정적 크로스빌드 완료, 실장비 실행 확인 필요 |
| [`HowAmI-Linux-arm64`](release/dist/v0.1.0/HowAmI-Linux-arm64) | Linux ARM64 musl | 정적 크로스빌드 완료, 실장비 실행 확인 필요 |

체크섬: [`SHA256SUMS.txt`](release/dist/v0.1.0/SHA256SUMS.txt)  
빌드 정보: [`BUILD-INFO.md`](release/dist/v0.1.0/BUILD-INFO.md)

Windows ARM64 및 macOS 바이너리는 아직 포함되어 있지 않습니다. 소스는 해당 플랫폼 Collector를 포함하고 있으며, 각 대상 환경에서 직접 빌드할 수 있습니다.

### 문서

- [빌드](docs/BUILD.md)
- [아키텍처](docs/ARCHITECTURE.md)
- [개인정보](docs/PRIVACY.md)
- [보안](SECURITY.md)
- [릴리스](docs/RELEASE.md)

---

## English

### Features

- Collect as much system information as the OS and devices expose in one run
- Inspect CPU, GPU, mainboard, BIOS/UEFI, RAM, storage, displays, network, audio, USB, Bluetooth, and more
- Collect driver, firmware, and device-identification data
- Generate TXT and JSON together
- Never guess values that cannot be discovered reliably
- Use a separate Administrator/root process only when elevated collection is needed
- No report upload, remote analysis, or telemetry

### Supported platforms

#### Windows

- Windows version/build and system manufacturer/model
- CPU, GPU, GPU driver, resolution/refresh rate
- mainboard and BIOS/UEFI
- RAM module capacity/speed/part number/serial
- disks, physical disks, volumes, firmware, health/status
- monitors and WMI EDID
- network, audio, USB, Bluetooth, battery
- PnP devices plus driver version/INF/signing information
- TPM and Secure Boot

#### macOS

- macOS version/build and kernel
- Hardware / Display & GPU / Memory / Storage / NVMe
- Audio / USB / Network / Bluetooth / Power
- PCI / Thunderbolt & USB4 / Extensions
- System Extensions

#### Linux

- distribution/kernel/CPU
- mainboard/BIOS/DMI
- RAM and memory-module detail
- storage/filesystem/UUID/model/serial/firmware
- PCI/USB devices and kernel drivers
- GPU/DRM cards
- monitor connection state/modes/EDID
- network adapters
- UEFI/legacy boot, Secure Boot, TPM where exposed
- power/battery and hwmon sensors
- ALSA audio, input devices, Bluetooth controllers
- optional `lspci`, `lsusb`, and `dmidecode` detail when installed

> No operating system can guarantee discovery of every physical component. Devices such as conventional PSUs, or values not exposed by firmware/drivers, cannot be identified reliably in software.

### Output

By default HowAmI writes two files to the Desktop:

```text
HowAmI_Report_YYYYMMDD_HHMMSS_mmm.txt
HowAmI_Report_YYYYMMDD_HHMMSS_mmm.json
```

TXT is optimized for human reading. JSON contains structured data suitable for automation or other tools and exposes its format version through `meta.schema_version`.

### Usage

```text
HowAmI
HowAmI --output <directory>
HowAmI --no-elevate
```

- Default: request Administrator/root access when useful for detailed collection.
- `--output`: choose a custom output directory.
- `--no-elevate`: collect only what is available without privilege elevation.

### Privacy

Reports can contain **unique identifiers** such as hardware serial numbers, system UUIDs, MAC addresses, host names, device IDs, and volume UUIDs. HowAmI does not transmit collected data anywhere.

Review reports before sharing them with another person or service.

See [`docs/PRIVACY.md`](docs/PRIVACY.md).

### Build

Stable Rust `1.74+` is required.

```bash
cargo build --release --locked
```

Artifacts:

- Windows: `target/release/HowAmI.exe`
- macOS/Linux: `target/release/HowAmI`

See [`docs/BUILD.md`](docs/BUILD.md) for platform-specific instructions.

### Available v0.1.0 binaries

| File | Target | Status |
| --- | --- | --- |
| [`HowAmI-Windows-x64.exe`](release/dist/v0.1.0/HowAmI-Windows-x64.exe) | Windows x64 | Built and basic-run checked on Windows x64 |
| [`HowAmI-Linux-x64`](release/dist/v0.1.0/HowAmI-Linux-x64) | Linux x64 musl | Static cross-build complete; target-hardware run still recommended |
| [`HowAmI-Linux-arm64`](release/dist/v0.1.0/HowAmI-Linux-arm64) | Linux ARM64 musl | Static cross-build complete; target-hardware run still recommended |

Checksums: [`SHA256SUMS.txt`](release/dist/v0.1.0/SHA256SUMS.txt)  
Build details: [`BUILD-INFO.md`](release/dist/v0.1.0/BUILD-INFO.md)

Windows ARM64 and macOS binaries are not currently included. Their collectors are present in source and can be built on the corresponding target environments.

### Documentation

- [Build](docs/BUILD.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Privacy](docs/PRIVACY.md)
- [Security](SECURITY.md)
- [Release](docs/RELEASE.md)

## License

MIT License. See [`LICENSE`](LICENSE).
