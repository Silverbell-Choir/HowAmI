# HowAmI

> **One click. One report. Everything your computer can expose.**
>
> **한 번 실행. 하나의 리포트. 컴퓨터가 제공할 수 있는 시스템 정보를 최대한 한곳에.**

HowAmI는 Windows, macOS, Linux에서 하드웨어·펌웨어·드라이버·운영체제 정보를 수집해 사람이 읽기 쉬운 TXT와 구조화된 JSON 리포트로 저장하는 오픈소스 도구입니다.

HowAmI is an open-source Windows, macOS, and Linux utility that collects hardware, firmware, driver, and operating-system information and writes both a human-readable TXT report and a structured JSON report.

> **Development status / 개발 상태:** early development (`v0.1.x`). The repository is currently private while report contents are reviewed for privacy and identifier exposure. / 현재 초기 개발 단계이며, 생성 리포트에 포함되는 개인정보·고유 식별 정보 검토가 끝날 때까지 저장소를 비공개로 유지합니다.

## 한국어

### 목표

- 실행 한 번으로 가능한 많은 시스템 정보를 수집합니다.
- Windows는 관리자 권한, macOS/Linux는 가능한 경우 관리자/root 권한을 요청합니다.
- 수집 결과는 기본적으로 바탕화면에 `HowAmI_Report_*.txt`와 `HowAmI_Report_*.json`으로 생성합니다.
- HowAmI 자체에는 서버 업로드 기능이나 원격 전송 기능을 구현하지 않습니다.
- OS/펌웨어/장치가 제공하지 않는 정보는 추측하지 않습니다.

### 현재 수집 범위

**Windows**

- OS / 시스템 모델
- CPU, GPU
- 메인보드, BIOS/UEFI
- RAM 모듈
- 디스크, 볼륨, 펌웨어/상태(운영체제가 제공하는 범위)
- 모니터 및 WMI EDID 정보
- 네트워크, 오디오, USB, Bluetooth, 배터리
- 현재 연결된 PnP 장치
- 서명된 PnP 드라이버와 드라이버 버전
- TPM / Secure Boot 정보(조회 가능한 경우)

**macOS**

- `system_profiler`가 제공하는 Hardware, Display/GPU, Memory, Storage/NVMe, Audio, USB, Network, Bluetooth, Power, PCI, Thunderbolt/USB4, Extension 정보
- macOS 버전과 커널 정보
- System Extension 목록(조회 가능한 경우)

**Linux**

- `/proc`, `/sys`, DMI sysfs 기반 OS/CPU/메인보드/BIOS/메모리/PCI/USB/네트워크/DRM 정보
- `lsblk` 기반 스토리지 정보
- 설치되어 있는 경우 `lspci`, `lsusb`, `dmidecode`의 추가 정보

> 어떤 OS에서도 “모든 물리 부품을 100% 식별”하는 것은 보장할 수 없습니다. PSU처럼 소프트웨어 인터페이스를 제공하지 않는 장치나 펌웨어/드라이버가 노출하지 않는 값은 읽을 수 없습니다.

### 개인정보

현재 기본 리포트는 **개인 확인용 상세 리포트**를 목표로 하므로 다음과 같은 고유 식별 정보가 포함될 수 있습니다.

- 장치/보드/디스크/RAM 시리얼 번호
- 시스템 UUID
- MAC 주소
- 호스트명
- PnP/PCI/USB 장치 식별자

리포트는 로컬에서 생성되며 HowAmI가 외부 서버로 업로드하지 않습니다. 단, 생성 파일을 다른 사람에게 전달하기 전에는 내용을 직접 확인해야 합니다.

### 빌드

Rust 툴체인이 필요합니다.

```bash
cargo build --release
```

산출물:

- Windows: `target/release/HowAmI.exe`
- macOS/Linux: `target/release/HowAmI`

자동 CI/GitHub Actions는 사용하지 않습니다. 각 운영체제에서 로컬로 빌드·검증합니다.

자세한 내용은 [`docs/BUILD.md`](docs/BUILD.md)를 참고하세요.

---

## English

### Goals

- Collect as much system information as the operating system and devices can expose with one run.
- Request Administrator privileges on Windows and Administrator/root privileges on macOS/Linux where possible.
- Write `HowAmI_Report_*.txt` and `HowAmI_Report_*.json` to the Desktop by default.
- Do not implement server upload or remote transmission in HowAmI itself.
- Never guess values that the OS, firmware, or device does not expose.

### Current collection scope

**Windows**

- OS and system model
- CPU and GPU
- Mainboard and BIOS/UEFI
- Physical memory modules
- Disks, volumes, firmware/status where exposed by Windows
- Monitors and WMI EDID data
- Network, audio, USB, Bluetooth, and battery devices
- Currently present PnP devices
- Signed PnP drivers and driver versions
- TPM and Secure Boot where available

**macOS**

- `system_profiler` Hardware, Display/GPU, Memory, Storage/NVMe, Audio, USB, Network, Bluetooth, Power, PCI, Thunderbolt/USB4, and Extension data
- macOS version and kernel information
- System Extension list where available

**Linux**

- OS/CPU/mainboard/BIOS/memory/PCI/USB/network/DRM data from `/proc`, `/sys`, and DMI sysfs
- Storage data from `lsblk`
- Additional data from `lspci`, `lsusb`, and `dmidecode` when those tools are installed

> No operating system can guarantee identification of every physical component. Devices such as conventional PSUs, or values not exposed by firmware/drivers, cannot be discovered reliably in software.

### Privacy

The current default report is intentionally detailed for personal inspection and may contain unique identifiers such as:

- device/board/disk/RAM serial numbers
- system UUID
- MAC addresses
- host name
- PnP/PCI/USB identifiers

Reports are generated locally and HowAmI does not upload them to any server. Review a report before sharing it with another person or service.

### Build

A Rust toolchain is required.

```bash
cargo build --release
```

Artifacts:

- Windows: `target/release/HowAmI.exe`
- macOS/Linux: `target/release/HowAmI`

No automated CI or GitHub Actions are used. Builds and validation are performed locally on each target operating system.

See [`docs/BUILD.md`](docs/BUILD.md) for details.

## License

MIT License. See [`LICENSE`](LICENSE).
