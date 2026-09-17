# HowAmI

> **One click. One report. Everything your computer can expose.**  
> **한 번 실행. 하나의 리포트. 컴퓨터가 제공할 수 있는 시스템 정보를 최대한 한곳에.**

HowAmI는 Windows, macOS, Linux에서 하드웨어·펌웨어·드라이버·운영체제 정보를 로컬로 수집하고, 사람이 읽기 쉬운 TXT와 구조화된 JSON 리포트로 저장하는 오픈소스 도구입니다.

HowAmI is an open-source Windows, macOS, and Linux utility that locally collects hardware, firmware, driver, and operating-system information and writes both a human-readable TXT report and structured JSON.

> **Development status / 개발 상태:** `v0.1.x`. The repository remains private until real-device output is reviewed for privacy and identifier exposure. / 실제 장비 출력의 개인정보·고유 식별 정보 검토가 끝날 때까지 저장소를 비공개로 유지합니다.

---

## 한국어

### 목적

- 실행 한 번으로 OS와 장치가 노출하는 시스템 정보를 최대한 수집합니다.
- 값이 없거나 확인할 수 없으면 추측하지 않습니다.
- 결과는 기본적으로 사용자의 Desktop에 `HowAmI_Report_*.txt`와 `HowAmI_Report_*.json`으로 생성합니다.
- TXT는 용량·네트워크 속도·일부 하드웨어 코드에 사람이 읽기 쉬운 보조 표기를 제공합니다.
- JSON은 수집된 원시 값을 최대한 유지하며 `meta.schema_version`으로 형식 버전을 표시합니다.
- HowAmI에는 서버 업로드, 원격 분석, 텔레메트리 기능을 구현하지 않습니다.
- 관리자/root 권한이 필요한 수집은 별도 elevated child에서만 수행하고 최종 리포트는 일반 사용자 프로세스가 작성합니다.

### 현재 수집 범위

**Windows**

- Windows 버전/빌드/시스템 모델
- CPU, GPU, GPU 드라이버, 현재 해상도/주사율
- GPU VRAM 보조 조회(Windows 레지스트리가 제공하는 경우)
- 메인보드, BIOS/UEFI
- RAM 모듈/속도/파트넘버/시리얼
- 디스크/물리 디스크/볼륨/펌웨어/상태
- 모니터 및 WMI EDID
- 네트워크, 오디오, USB, Bluetooth, 배터리
- 현재 연결된 PnP 장치
- PnP 드라이버 버전/INF/서명 정보
- TPM / Secure Boot

**macOS**

- macOS 버전과 커널
- `system_profiler`의 Hardware, Display/GPU, Memory, Storage/NVMe, Audio, USB, Network, Bluetooth, Power, PCI, Thunderbolt/USB4, Extension 정보
- System Extension 목록
- 각 `system_profiler` data type을 독립 수집하여 일부 실패가 전체 수집을 중단하지 않도록 처리

**Linux**

- 배포판/커널/CPU
- 메인보드/BIOS/DMI
- RAM 및 `dmidecode` 메모리 모듈 정보(사용 가능한 경우)
- 스토리지/파일시스템/UUID/모델/시리얼/펌웨어 revision
- PCI/USB 장치와 커널 드라이버/드라이버 버전
- GPU/DRM 카드
- 모니터 연결 상태/모드/EDID 제조사·제품·시리얼
- 네트워크 어댑터
- 전원/배터리
- hwmon 온도/팬/전압/전력 등 커널이 노출하는 센서 값
- ALSA 오디오, 입력 장치, Bluetooth 컨트롤러
- 설치되어 있는 경우 `lspci`, `lsusb`, `dmidecode` 추가 정보

> 어떤 OS에서도 모든 물리 부품을 100% 식별할 수 있다고 보장하지 않습니다. 일반 PSU처럼 소프트웨어 인터페이스가 없거나 펌웨어/드라이버가 값을 노출하지 않는 장치는 확인할 수 없습니다.

### 권한과 보안

HowAmI를 일반 권한으로 실행하면 부모 프로세스는 그대로 일반 사용자 권한을 유지합니다. 추가 권한이 필요할 때만 별도 관리자/root child가 하드웨어 수집을 수행하고 인증된 임시 handoff 파일로 결과를 돌려준 뒤 종료합니다. 최종 TXT/JSON은 사용자 프로세스가 작성합니다.

관리자/root 환경에서 PATH 검색으로 다른 실행파일이 선택되는 위험을 줄이기 위해 외부 시스템 도구는 신뢰된 시스템 경로만 사용합니다. 수집용 외부 프로세스에는 timeout도 적용합니다.

### 개인정보

상세 리포트에는 다음 값이 포함될 수 있습니다.

- 장치/보드/BIOS/RAM/스토리지/모니터 시리얼
- 시스템 UUID
- MAC 주소
- 호스트명
- PnP/PCI/USB 장치 식별자
- 볼륨/파일시스템 UUID
- 입력 장치 Unique ID
- 드라이버/INF 식별 정보

HowAmI가 이 정보를 네트워크로 전송하지는 않습니다. 다른 사람이나 서비스에 리포트를 전달하기 전에는 사용자가 직접 내용을 확인해야 합니다.

### 사용

기본 실행:

```text
HowAmI
```

출력 위치 지정:

```text
HowAmI --output <directory>
```

권한 상승 없이 실행:

```text
HowAmI --no-elevate
```

### 빌드

Rust `1.74+` stable toolchain이 필요합니다.

```bash
cargo build --release
```

기본 산출물:

- Windows: `target/release/HowAmI.exe`
- macOS/Linux: `target/release/HowAmI`

GitHub Actions/유료 CI는 사용하지 않습니다. 각 대상 OS/아키텍처에서 로컬 빌드와 실제 장비 검증을 수행합니다.

자세한 내용: [`docs/BUILD.md`](docs/BUILD.md)  
아키텍처: [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)  
개인정보: [`docs/PRIVACY.md`](docs/PRIVACY.md)  
보안: [`SECURITY.md`](SECURITY.md)  
수동 릴리스 절차: [`docs/RELEASE.md`](docs/RELEASE.md)

---

## English

### Purpose

- Collect as much system information as the OS and devices expose in one run.
- Never guess a value that cannot be discovered reliably.
- Write `HowAmI_Report_*.txt` and `HowAmI_Report_*.json` to the user's Desktop by default.
- Add human-readable annotations in TXT for capacities, network rates, and selected hardware codes.
- Preserve discovered values as closely as practical in JSON and expose the format version as `meta.schema_version`.
- Do not implement server upload, remote analysis, or telemetry.
- Perform Administrator/root-only collection in a separate elevated child and write final reports from the normal user process.

### Current collection scope

**Windows**

- Windows version/build and system model
- CPU, GPU, GPU driver, current resolution/refresh rate
- registry-assisted GPU VRAM where Windows exposes it
- mainboard and BIOS/UEFI
- physical RAM modules, speed, part number, serial
- disks, physical disks, volumes, firmware, health/status
- monitors and WMI EDID
- network, audio, USB, Bluetooth, and battery devices
- currently present PnP devices
- PnP driver versions, INF files, and signing information
- TPM and Secure Boot

**macOS**

- macOS version and kernel
- `system_profiler` Hardware, Display/GPU, Memory, Storage/NVMe, Audio, USB, Network, Bluetooth, Power, PCI, Thunderbolt/USB4, and Extension data
- System Extension list
- independent collection per `system_profiler` data type so one failure does not discard the rest

**Linux**

- distribution/kernel/CPU
- mainboard/BIOS/DMI
- RAM plus `dmidecode` memory-module detail when available
- storage/filesystems/UUID/model/serial/firmware revision
- PCI/USB devices and kernel driver/version information
- GPU/DRM cards
- monitor connection modes and EDID manufacturer/product/serial data
- network adapters
- power supplies and batteries
- kernel-exposed hwmon temperature/fan/voltage/power sensors
- ALSA audio, input devices, and Bluetooth controllers
- additional `lspci`, `lsusb`, and `dmidecode` output when installed

> No operating system can guarantee discovery of every physical component. Devices such as conventional PSUs, or values not exposed by firmware/drivers, cannot be identified reliably in software.

### Privileges and security

When HowAmI starts as a normal user, the parent process stays unprivileged. A separate Administrator/root child performs only the collection that benefits from elevation, returns the result through an authenticated temporary handoff file, and exits. The normal user process writes the final TXT/JSON files.

External helper programs are launched only from trusted system locations rather than arbitrary PATH matches while elevated. Collector subprocesses also have timeouts.

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

HowAmI does not transmit these values over the network. Review a report yourself before sharing it with another person or service.

### Usage

Default:

```text
HowAmI
```

Custom output directory:

```text
HowAmI --output <directory>
```

Disable privilege elevation:

```text
HowAmI --no-elevate
```

### Build

A stable Rust `1.74+` toolchain is required.

```bash
cargo build --release
```

Default artifacts:

- Windows: `target/release/HowAmI.exe`
- macOS/Linux: `target/release/HowAmI`

No GitHub Actions or paid CI are used. Builds and physical-device validation are performed locally for each target OS/architecture.

Details: [`docs/BUILD.md`](docs/BUILD.md)  
Architecture: [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)  
Privacy: [`docs/PRIVACY.md`](docs/PRIVACY.md)  
Security: [`SECURITY.md`](SECURITY.md)  
Manual release procedure: [`docs/RELEASE.md`](docs/RELEASE.md)

## License

MIT License. See [`LICENSE`](LICENSE).
