# Architecture / 아키텍처

## 한국어

HowAmI는 **공통 데이터 모델 + OS별 Collector + 공통 Renderer** 구조를 사용합니다.

```text
HowAmI
├─ command          신뢰된 시스템 실행파일 경로 및 timeout 실행
├─ elevation        관리자/root 수집 handoff
├─ edid             Linux DRM EDID parser
├─ platform
│  ├─ windows       PowerShell/CIM/WMI/PnP 기반 수집
│  ├─ macos         system_profiler 및 시스템 도구 기반 수집
│  └─ linux         /proc, /sys 및 선택적 시스템 도구 기반 수집
├─ model            OS 독립 Section/Record/Field 모델
├─ report           TXT renderer 및 사람이 읽는 단위 표기
└─ output           TXT/JSON 저장 및 Desktop 경로 처리
```

### 데이터 흐름

```text
OS / Firmware / Device APIs
          │
          ▼
Platform Collector
          │
          ▼
Collection
  ├─ Section
  │   └─ DeviceRecord
  │       └─ Field
  └─ Warnings
          │
          ├───────────────┐
          ▼               ▼
      TXT Renderer     JSON Serializer
          │               │
          └───────┬───────┘
                  ▼
             Local files
```

각 OS는 서로 다른 방식으로 정보를 수집하지만 최종적으로 같은 `Section -> DeviceRecord -> Field` 구조로 정규화합니다.

### 권한 모델

일반 사용자로 시작한 경우 부모 프로세스는 일반 권한을 유지합니다. 추가 권한이 필요하면 별도 elevated child가 수집만 수행하고, 인증된 임시 handoff를 통해 결과를 부모 프로세스에 반환합니다.

```text
User process
   │
   ├─ create private handoff + token
   ├─ request elevation
   │       │
   │       └─ Elevated child
   │            ├─ authenticate handoff
   │            ├─ collect
   │            └─ write collection JSON
   │
   ├─ read/delete handoff
   └─ render/write TXT + JSON
```

이 구조를 통해 높은 권한으로 실행되는 범위를 수집 단계로 제한하고, 일반적인 실행에서는 최종 리포트를 사용자 권한으로 작성합니다.

권한 상승이 거부되거나 실패하면 일반 권한으로 가능한 정보를 수집하고 `warnings`에 상태를 기록합니다. `--no-elevate`를 사용하면 권한 상승을 요청하지 않습니다.

### 외부 명령 실행

관리자/root 상태에서 임의 PATH 검색을 사용하지 않도록 시스템 도구를 신뢰된 위치에서만 실행합니다.

- Windows: Windows API로 시스템 디렉터리를 확인한 뒤 PowerShell 실행
- macOS: `/usr/bin`, `/usr/sbin`의 고정 시스템 도구 사용
- Linux: `/usr/bin`, `/bin`, `/usr/sbin`, `/sbin`에서만 선택적 helper 탐색

외부 수집 프로세스에는 timeout을 적용합니다.

### 플랫폼별 Collector

#### Windows

주요 소스:

- CIM / WMI
- PnP
- Windows Storage cmdlets
- Windows Registry
- Windows Known Folder API

수집 범위에는 시스템/CPU/GPU/메인보드/BIOS/RAM/스토리지/모니터/네트워크/오디오/USB/Bluetooth/배터리/드라이버/TPM/Secure Boot 등이 포함됩니다.

#### macOS

주요 소스:

- `sw_vers`
- `uname`
- `system_profiler`
- `systemextensionsctl`

`system_profiler`는 data type별로 독립 실행하여 한 항목의 실패가 다른 수집까지 중단시키지 않도록 합니다.

#### Linux

주요 소스:

- `/proc`
- `/sys`
- DMI sysfs
- DRM / EDID
- power_supply / hwmon / ALSA / input / Bluetooth sysfs
- `lsblk`
- 선택적 `lspci`, `lsusb`, `dmidecode`

기본 커널 인터페이스를 우선 사용하고, 배포판에 따라 존재하지 않을 수 있는 명령은 보조 데이터 소스로 취급합니다.

### 출력 모델

JSON은 수집된 값을 가능한 한 원형에 가깝게 문자열 형태로 보존하고 `meta.schema_version`으로 형식 버전을 표시합니다.

TXT는 같은 데이터를 사용하지만 사람이 읽기 쉽도록 명확한 단위가 있는 값에 보조 표기를 추가할 수 있습니다.

```text
VRAMBytes          : 8589934592 (8.00 GiB)
SpeedBitsPerSecond : 1000000000 (1.00 Gbit/s)
```

### 설계 원칙

1. **추측 금지** — 장치나 OS가 제공하지 않는 값을 임의 생성하지 않습니다.
2. **부분 실패 허용** — 일부 장치/API 조회 실패가 전체 리포트 생성을 중단시키지 않도록 합니다.
3. **로컬 처리** — 수집과 리포트 생성 과정에 네트워크 전송을 사용하지 않습니다.
4. **공통 데이터 모델** — OS별 차이를 공통 출력 구조로 정규화합니다.
5. **네이티브 정보 우선** — 각 OS가 제공하는 기본 시스템 인터페이스를 우선 사용합니다.
6. **권한 최소화** — 높은 권한은 필요한 수집 단계에만 사용합니다.
7. **구조 안정성** — JSON schema version으로 형식 변경을 구분합니다.

---

## English

HowAmI uses a **shared data model + OS-specific collectors + shared renderers** architecture.

```text
HowAmI
├─ command          trusted executable paths and timeout execution
├─ elevation        Administrator/root collection handoff
├─ edid             Linux DRM EDID parser
├─ platform
│  ├─ windows       PowerShell/CIM/WMI/PnP collection
│  ├─ macos         system_profiler and system-tool collection
│  └─ linux         /proc, /sys, and optional system-tool collection
├─ model            OS-independent Section/Record/Field model
├─ report           TXT renderer and human-readable unit annotations
└─ output           TXT/JSON output and Desktop path handling
```

### Data flow

```text
OS / Firmware / Device APIs
          │
          ▼
Platform Collector
          │
          ▼
Collection
  ├─ Section
  │   └─ DeviceRecord
  │       └─ Field
  └─ Warnings
          │
          ├───────────────┐
          ▼               ▼
      TXT Renderer     JSON Serializer
          │               │
          └───────┬───────┘
                  ▼
             Local files
```

Each operating system uses different collection mechanisms, but all results are normalized into the same `Section -> DeviceRecord -> Field` structure.

### Privilege model

When started by a normal user, the parent remains unprivileged. If additional privileges are useful, a separate elevated child performs collection only and returns data through an authenticated temporary handoff.

```text
User process
   │
   ├─ create private handoff + token
   ├─ request elevation
   │       │
   │       └─ Elevated child
   │            ├─ authenticate handoff
   │            ├─ collect
   │            └─ write collection JSON
   │
   ├─ read/delete handoff
   └─ render/write TXT + JSON
```

This limits elevated execution to collection and keeps final report writing in the original user process during the normal flow.

If elevation is declined or fails, HowAmI continues with the information available to the current user and records the condition in `warnings`. `--no-elevate` disables the elevation request.

### External command execution

System tools are launched only from trusted locations rather than arbitrary PATH matches in Administrator/root contexts.

- Windows: resolve the system directory through the Windows API and launch PowerShell from there
- macOS: use fixed tools under `/usr/bin` and `/usr/sbin`
- Linux: discover optional helpers only under `/usr/bin`, `/bin`, `/usr/sbin`, and `/sbin`

External collector processes use timeouts.

### Platform collectors

#### Windows

Primary sources:

- CIM / WMI
- PnP
- Windows Storage cmdlets
- Windows Registry
- Windows Known Folder API

Collection includes system/CPU/GPU/mainboard/BIOS/RAM/storage/display/network/audio/USB/Bluetooth/battery/driver/TPM/Secure Boot data where exposed by Windows.

#### macOS

Primary sources:

- `sw_vers`
- `uname`
- `system_profiler`
- `systemextensionsctl`

`system_profiler` data types are collected independently so failure of one category does not discard the others.

#### Linux

Primary sources:

- `/proc`
- `/sys`
- DMI sysfs
- DRM / EDID
- power_supply / hwmon / ALSA / input / Bluetooth sysfs
- `lsblk`
- optional `lspci`, `lsusb`, and `dmidecode`

Kernel interfaces are preferred. Commands that may not exist on every distribution are treated as optional supplementary sources.

### Output model

JSON preserves collected values as strings as closely as practical and exposes the format version through `meta.schema_version`.

TXT uses the same data but may add human-readable annotations for values with clear units.

```text
VRAMBytes          : 8589934592 (8.00 GiB)
SpeedBitsPerSecond : 1000000000 (1.00 Gbit/s)
```

### Design principles

1. **No guessing** — never manufacture values the device or OS does not expose.
2. **Partial failure tolerance** — failure of one device/API should not stop the remaining report where possible.
3. **Local processing** — collection and report generation do not require network transmission.
4. **Shared data model** — normalize platform differences into a common output structure.
5. **Prefer native sources** — use the operating system's own system interfaces first.
6. **Least privilege** — use elevated privileges only for the collection steps that need them.
7. **Schema stability** — distinguish format changes with a JSON schema version.
