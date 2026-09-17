# Privacy / 개인정보

## 한국어

HowAmI는 사용자가 **자기 컴퓨터의 상세 시스템 정보를 확인하는 도구**입니다. 상세 리포트에는 장치와 시스템을 식별할 수 있는 값이 포함될 수 있습니다.

### 포함될 수 있는 정보

- 메인보드/시스템/BIOS 시리얼 및 UUID
- RAM/스토리지/모니터/USB 장치 시리얼 번호
- MAC 주소
- 컴퓨터 호스트명
- PnP/PCI/USB Instance ID 및 Device ID
- 볼륨/파일시스템 UUID
- 입력 장치 Unique ID
- 드라이버/INF 식별 정보
- 운영체제와 시스템 도구가 제공하는 기타 장치 식별자

### 데이터 처리 방식

- 수집은 로컬 컴퓨터에서 수행됩니다.
- HowAmI에는 리포트 업로드, 원격 분석, 텔레메트리 기능이 없습니다.
- 생성된 TXT/JSON은 기본적으로 Desktop 또는 `--output`으로 지정한 로컬 경로에 저장됩니다.
- HowAmI는 수집 결과를 외부 서버로 전송하지 않습니다.

### 관리자/root 권한

일반 사용자로 시작한 경우 관리자/root child는 필요한 시스템 정보 수집만 수행하고 종료합니다. 최종 TXT/JSON은 일반적인 실행 흐름에서 원래 사용자 프로세스가 작성합니다.

### 리포트 공유 시 주의

HowAmI 리포트는 개인 확인용 상세 정보를 포함할 수 있으므로, 다른 사람·커뮤니티·지원 서비스·AI 서비스 등에 공유하기 전에 내용을 직접 확인하세요.

특히 다음 값은 장치나 시스템을 장기간 식별하는 데 사용될 수 있습니다.

```text
Serial Number
UUID
MAC Address
Host Name
Device / Instance ID
Filesystem / Volume UUID
```

필요한 항목만 전달하거나 민감하다고 판단한 값을 직접 제거한 뒤 공유하는 것을 권장합니다.

---

## English

HowAmI is designed for **detailed inspection of the user's own computer**. Reports can contain values that identify specific hardware or a specific system installation.

### Information that may appear

- motherboard/system/BIOS serials and UUIDs
- RAM/storage/monitor/USB serial numbers
- MAC addresses
- computer host name
- PnP/PCI/USB instance and device IDs
- volume/filesystem UUIDs
- input-device unique IDs
- driver/INF identifiers
- other device identifiers exposed by the operating system and system tools

### Data handling

- Collection is performed locally on the computer.
- HowAmI does not implement report upload, remote analysis, or telemetry.
- TXT/JSON reports are written to the Desktop by default or to a local path selected with `--output`.
- HowAmI does not transmit collected report data to an external server.

### Administrator/root access

When started by a normal user, the Administrator/root child performs only the collection that requires additional privileges and then exits. In the normal execution flow, the original user process writes the final TXT/JSON reports.

### Before sharing a report

A HowAmI report can contain detailed identifiers intended for personal inspection. Review it before sharing it with another person, a community, a support service, or an AI service.

The following values can be especially useful for identifying a device or installation over time:

```text
Serial Number
UUID
MAC Address
Host Name
Device / Instance ID
Filesystem / Volume UUID
```

Share only the information you need, and remove any values you consider sensitive before sending the report elsewhere.
