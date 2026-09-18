# Security / 보안

## 한국어

HowAmI는 상세 하드웨어 정보를 읽기 위해 관리자/root 권한을 사용할 수 있으므로, 권한 상승과 외부 명령 실행을 보수적으로 처리합니다.

### 보안 원칙

- 리포트 업로드·원격 분석·텔레메트리 기능 없음
- 일반 사용자로 시작한 경우 최종 TXT/JSON은 일반 사용자 프로세스가 작성
- 관리자/root child는 시스템 정보 수집과 handoff 작성만 수행 후 종료
- elevated 상태에서 임의 PATH 검색을 사용하지 않음
- Windows PowerShell은 Windows 시스템 디렉터리를 API로 확인한 뒤 실행
- macOS 시스템 도구는 고정된 `/usr/bin` 또는 `/usr/sbin` 경로 사용
- Linux 보조 도구는 `/usr/bin`, `/bin`, `/usr/sbin`, `/sbin`에서만 탐색
- 외부 수집 명령에 timeout 적용
- 장치가 제공하지 않는 값은 추측하지 않음

### 권한 상승 흐름

```text
User process
   │
   ├─ create private handoff + token
   ├─ request elevation
   │       │
   │       └─ Administrator/root child
   │            ├─ verify privilege
   │            ├─ authenticate handoff
   │            ├─ collect system data
   │            └─ write collection JSON
   │
   ├─ read/delete handoff
   └─ write final TXT/JSON
```

### 임시 handoff 파일

부모 프로세스는 고유한 handoff 파일을 `create_new`로 생성합니다. Unix 계열에서는 `0600` 권한으로 제한하고, 이번 권한 상승 흐름에 대응하는 token을 기록합니다.

관리자/root child는 다음 조건을 확인한 뒤에만 결과를 기록합니다.

1. 실제 관리자/root 권한인지
2. 전달된 경로가 이미 존재하는 일반 파일인지
3. symbolic link가 아닌지
4. 저장된 token과 전달된 token이 일치하는지
5. HowAmI handoff marker 형식이 맞는지

검증 후에는 **검증에 사용한 동일 열린 파일 핸들**을 truncate/write에 재사용합니다. 검증 이후 경로를 다시 열지 않아 경로 교체에 의한 TOCTOU 위험을 줄입니다.

이 token은 암호학적 비밀키가 아니라, 숨겨진 내부 인자만 임의 호출해 elevated HowAmI가 무관한 파일을 덮어쓰는 것을 방지하기 위한 handoff 인증 표식입니다.

### 네트워크

HowAmI의 수집 및 리포트 생성 흐름에는 네트워크 전송 기능이 없습니다. 생성된 리포트는 로컬 파일로만 저장됩니다.

### 보안 문제 제보

보안 취약점을 발견한 경우 민감한 재현 정보나 HowAmI 리포트를 공개 Issue에 그대로 첨부하지 마세요. 비공개 보안 제보 채널이 제공되는 경우 해당 채널을 우선 사용하세요.

---

## English

HowAmI may use Administrator/root privileges to obtain detailed hardware information, so privilege elevation and external process execution are treated as security-sensitive operations.

### Security principles

- no report upload, remote analysis, or telemetry
- when started as a normal user, final TXT/JSON output is written by the normal user process
- the Administrator/root child performs system collection and handoff writing only, then exits
- elevated code does not use arbitrary PATH-based executable discovery
- Windows PowerShell is launched from the Windows system directory resolved through the Windows API
- macOS system tools use fixed `/usr/bin` or `/usr/sbin` locations
- Linux helper tools are discovered only under `/usr/bin`, `/bin`, `/usr/sbin`, and `/sbin`
- external collector commands use timeouts
- values not exposed by the device/OS are not guessed

### Elevation flow

```text
User process
   │
   ├─ create private handoff + token
   ├─ request elevation
   │       │
   │       └─ Administrator/root child
   │            ├─ verify privilege
   │            ├─ authenticate handoff
   │            ├─ collect system data
   │            └─ write collection JSON
   │
   ├─ read/delete handoff
   └─ write final TXT/JSON
```

### Temporary handoff file

The parent creates a unique handoff file with `create_new`. On Unix-like systems it is restricted to mode `0600`, and a token associated with that elevation flow is written before privilege elevation.

The Administrator/root child writes results only after verifying:

1. it is actually running with Administrator/root privileges,
2. the supplied path already refers to a regular file,
3. the path is not a symbolic link,
4. the stored token matches the token supplied to the child,
5. the value uses the expected HowAmI handoff marker format.

After validation, the child reuses **the same already-open authenticated file handle** for truncate/write. It does not reopen the path between validation and the privileged write, reducing path-replacement TOCTOU risk.

The token is not intended as a cryptographic secret. It is an authentication marker that prevents the hidden internal argument alone from being used to make an elevated HowAmI process overwrite an unrelated file.

### Network behavior

HowAmI's collection and report-generation flow contains no network transmission feature. Generated reports are stored only as local files.

### Reporting a security issue

Do not attach sensitive reproduction data or HowAmI reports directly to a public Issue. If a private security-reporting channel is available, use that channel instead.
