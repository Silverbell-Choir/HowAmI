# Architecture / 아키텍처

## 한국어

HowAmI는 **공통 리포트 모델 + OS별 Collector + 공통 Renderer** 구조를 사용합니다.

```text
HowAmI
├─ command          신뢰된 시스템 실행파일 경로 + timeout 실행
├─ elevation        최소 권한 관리자/root 수집 handoff
├─ edid             EDID base block 파서
├─ platform
│  ├─ windows       PowerShell/CIM/WMI/PnP 기반 수집
│  ├─ macos         system_profiler 및 시스템 도구 기반 수집
│  └─ linux         /proc, /sys 및 선택적 시스템 도구 기반 수집
├─ model            OS에 독립적인 Section/Record/Field 모델
├─ report           사람이 읽는 TXT 렌더러
└─ output           TXT/JSON 파일 저장 및 Desktop 경로 결정
```

### 권한 모델

일반 사용자가 HowAmI를 실행하면 **부모 프로세스는 일반 사용자 권한을 유지**합니다. 관리자/root 권한이 필요한 경우 별도의 elevated child를 실행하여 수집만 수행하고, 결과를 임시 handoff 파일로 부모 프로세스에 반환합니다.

최종 TXT/JSON은 일반 사용자 부모 프로세스가 작성합니다. 따라서 macOS/Linux에서 root 소유 리포트가 생성되는 문제를 피하고, 높은 권한으로 실행되는 코드 범위를 수집 단계로 제한합니다.

```text
User process
   │
   ├─ create private handoff file
   │
   ├─ request elevation
   │       │
   │       └─ Elevated child → collect → handoff JSON → exit
   │
   ├─ read/delete handoff
   └─ render TXT/JSON as the user
```

권한 요청이 거부되거나 실패하면 일반 권한 수집으로 계속하며 리포트 `warnings`에 기록합니다. `--no-elevate`를 지정하면 처음부터 권한 상승을 요청하지 않습니다.

### 외부 명령 실행

관리자/root 컨텍스트에서 PATH 기반 실행파일 검색을 피하기 위해 시스템 도구는 신뢰된 절대 경로만 사용합니다.

- Windows PowerShell: `%SystemRoot%\\System32\\WindowsPowerShell\\v1.0\\powershell.exe`
- macOS: `/usr/bin`, `/usr/sbin`의 고정 시스템 도구
- Linux: `/usr/bin`, `/bin`, `/usr/sbin`, `/sbin`에서만 보조 도구 탐색

수집용 외부 프로세스에는 timeout을 적용하여 WMI/system_profiler/lsusb 등 한 도구가 멈추더라도 HowAmI 전체가 무기한 대기하지 않도록 합니다.

### 설계 원칙

1. **추측 금지** — 장치가 제공하지 않는 값은 생성하지 않습니다.
2. **부분 실패 허용** — 하나의 장치/API 조회가 실패해도 전체 리포트 생성을 가능한 한 계속합니다.
3. **런타임 네트워크 통신 없음** — 하드웨어 조회와 파일 생성은 로컬에서만 수행합니다.
4. **공통 데이터 모델** — OS마다 수집 방식이 달라도 출력 구조는 `Section -> Record -> Field`로 통일합니다.
5. **OS 네이티브 정보 우선** — Windows는 CIM/WMI/PnP, macOS는 `system_profiler`, Linux는 `/proc`/`/sys`를 우선 사용합니다.
6. **선택적 시스템 도구** — Linux의 `lspci`, `lsusb`, `dmidecode`처럼 기본 설치가 보장되지 않는 도구는 존재할 때만 보조적으로 사용합니다.
7. **권한 최소화** — 최종 리포트 렌더링/저장은 사용자 권한에서 수행합니다.

### 현재 구현된 정규화/파서

- 공통 `Section -> DeviceRecord -> field` 모델
- TXT / JSON 출력
- Linux DRM EDID base block 파싱
- Windows CIM/WMI/PnP/드라이버 수집
- Windows GPU 레지스트리 VRAM 보조 조회
- macOS `system_profiler` data type별 독립 수집

### 이후 개선 가능 영역

- Windows/Linux 장치-드라이버 관계의 더 정교한 정규화
- SMART/NVMe 상태의 제조사 독립 정규화
- 필요 시 `--safe-share` 같은 별도 공유용 출력 프로필
- OS별 실장비 회귀 테스트 데이터셋

---

## English

HowAmI uses a **shared report model + OS-specific collectors + shared renderers** architecture.

```text
HowAmI
├─ command          trusted system executable paths + timeout execution
├─ elevation        least-privilege Administrator/root collection handoff
├─ edid             EDID base-block parser
├─ platform
│  ├─ windows       PowerShell/CIM/WMI/PnP collection
│  ├─ macos         system_profiler and system-tool collection
│  └─ linux         /proc, /sys, and optional system-tool collection
├─ model            OS-independent Section/Record/Field model
├─ report           human-readable TXT renderer
└─ output           TXT/JSON output and Desktop path resolution
```

### Privilege model

When a normal user starts HowAmI, the **parent process remains unprivileged**. If elevated collection is needed, HowAmI launches a separate elevated child that performs collection only and returns the result through a private temporary handoff file.

The final TXT/JSON files are written by the normal user process. This avoids root-owned reports on macOS/Linux and limits the amount of code running with elevated privileges.

```text
User process
   │
   ├─ create private handoff file
   │
   ├─ request elevation
   │       │
   │       └─ Elevated child → collect → handoff JSON → exit
   │
   ├─ read/delete handoff
   └─ render TXT/JSON as the user
```

If authorization is declined or fails, HowAmI falls back to standard-user collection and records the condition in report warnings. `--no-elevate` disables elevation entirely.

### External command execution

To avoid PATH-based executable substitution in an Administrator/root context, helper tools are launched only from trusted absolute system locations.

- Windows PowerShell: `%SystemRoot%\\System32\\WindowsPowerShell\\v1.0\\powershell.exe`
- macOS: fixed Apple system tools under `/usr/bin` and `/usr/sbin`
- Linux: optional helpers are discovered only under `/usr/bin`, `/bin`, `/usr/sbin`, and `/sbin`

Collector subprocesses use timeouts so a stalled WMI/system_profiler/lsusb operation cannot block HowAmI forever.

### Design principles

1. **No guessing** — never manufacture a value the device/OS does not expose.
2. **Partial failure tolerance** — one failed API/device query should not prevent the rest of the report where possible.
3. **No runtime network communication** — collection and report generation are local only.
4. **Common data model** — all platforms normalize into `Section -> Record -> Field`.
5. **Prefer native OS sources** — CIM/WMI/PnP on Windows, `system_profiler` on macOS, `/proc` and `/sys` on Linux.
6. **Optional system tools only** — non-guaranteed Linux utilities such as `lspci`, `lsusb`, and `dmidecode` are used only when present.
7. **Least privilege** — final report rendering and file output run under the user account.

### Implemented normalization/parsing

- shared `Section -> DeviceRecord -> field` model
- TXT / JSON output
- Linux DRM EDID base-block parsing
- Windows CIM/WMI/PnP/driver collection
- Windows registry-assisted GPU VRAM collection
- independent macOS `system_profiler` data-type collection

### Possible future improvements

- more precise Windows/Linux device-to-driver relationship normalization
- vendor-independent SMART/NVMe health normalization
- an optional share-oriented profile such as `--safe-share`
- physical-device regression datasets for each OS
