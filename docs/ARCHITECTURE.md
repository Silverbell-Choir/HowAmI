# Architecture / 아키텍처

## 한국어

HowAmI는 **공통 리포트 모델 + OS별 Collector + 공통 Renderer** 구조를 사용합니다.

```text
HowAmI
├─ elevation        관리자/root 권한 요청
├─ platform
│  ├─ windows       PowerShell/CIM/WMI/PnP 기반 수집
│  ├─ macos         system_profiler 및 시스템 도구 기반 수집
│  └─ linux         /proc, /sys 및 선택적 시스템 도구 기반 수집
├─ model            OS에 독립적인 Section/Record/Field 모델
├─ report           사람이 읽는 TXT 렌더러
└─ output           TXT/JSON 파일 저장 및 Desktop 경로 결정
```

### 설계 원칙

1. **추측 금지** — 장치가 제공하지 않는 값은 생성하지 않습니다.
2. **부분 실패 허용** — 하나의 장치/API 조회가 실패해도 전체 리포트 생성을 가능한 한 계속합니다.
3. **런타임 네트워크 통신 없음** — 하드웨어 조회와 파일 생성은 로컬에서만 수행합니다.
4. **공통 데이터 모델** — OS마다 수집 방식이 달라도 출력 구조는 `Section -> Record -> Field`로 통일합니다.
5. **OS 네이티브 정보 우선** — Windows는 CIM/WMI/PnP, macOS는 `system_profiler`, Linux는 `/proc`/`/sys`를 우선 사용합니다.
6. **선택적 외부 시스템 도구** — Linux의 `lspci`, `lsusb`, `dmidecode`처럼 기본 설치가 보장되지 않는 도구는 존재할 때만 보조적으로 사용합니다.

### 앞으로의 구조 확장

- EDID 정식 파서
- Windows/Linux 드라이버-장치 매핑 정규화
- SMART/NVMe 상태 정규화
- 민감정보 필터링 모드
- `--full`, `--safe-share` 같은 명시적 출력 프로필
- OS별 실장비 회귀 테스트

---

## English

HowAmI uses a **shared report model + OS-specific collectors + shared renderers** architecture.

```text
HowAmI
├─ elevation        Requests Administrator/root access
├─ platform
│  ├─ windows       PowerShell/CIM/WMI/PnP collection
│  ├─ macos         system_profiler and system-tool collection
│  └─ linux         /proc, /sys, and optional system-tool collection
├─ model            OS-independent Section/Record/Field model
├─ report           Human-readable TXT renderer
└─ output           TXT/JSON output and Desktop path resolution
```

### Design principles

1. **No guessing** — never manufacture a value the device/OS does not expose.
2. **Partial failure tolerance** — one failed API/device query should not prevent the rest of the report where possible.
3. **No runtime network communication** — collection and report generation are local only.
4. **Common data model** — all platforms normalize into `Section -> Record -> Field`.
5. **Prefer native OS sources** — CIM/WMI/PnP on Windows, `system_profiler` on macOS, `/proc` and `/sys` on Linux.
6. **Optional helper tools only** — non-guaranteed Linux utilities such as `lspci`, `lsusb`, and `dmidecode` are used only when present.

### Planned structural extensions

- proper EDID parser
- normalized Windows/Linux device-to-driver mapping
- normalized SMART/NVMe health reporting
- sensitive-identifier filtering mode
- explicit output profiles such as `--full` and `--safe-share`
- physical-device regression testing per OS
