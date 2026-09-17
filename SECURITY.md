# Security / 보안

## 한국어

HowAmI는 하드웨어 정보를 최대한 읽기 위해 관리자/root 권한을 사용할 수 있으므로, 권한 상승 코드와 외부 명령 실행을 보수적으로 설계합니다.

### 보안 원칙

- HowAmI 자체에는 네트워크 업로드/텔레메트리 기능을 구현하지 않습니다.
- 일반 사용자로 실행하면 최종 TXT/JSON 저장은 일반 사용자 프로세스가 수행합니다.
- 관리자/root child는 시스템 정보 수집과 handoff 작성만 수행하고 종료합니다.
- elevated 상태에서 임의 PATH 검색을 사용하지 않습니다.
- Windows PowerShell은 `%SystemRoot%\\System32\\WindowsPowerShell\\v1.0\\powershell.exe`를 사용합니다.
- macOS 시스템 도구는 고정된 `/usr/bin` 또는 `/usr/sbin` 경로를 사용합니다.
- Linux 보조 도구는 `/usr/bin`, `/bin`, `/usr/sbin`, `/sbin`에서만 찾습니다.
- 수집용 외부 명령에는 timeout을 적용합니다.
- 장치가 제공하지 않는 값은 추측하지 않습니다.

### 임시 handoff 파일

부모 프로세스가 먼저 임시 파일을 생성하고, Unix 계열에서는 `0600` 권한으로 제한합니다. elevated child는 이미 존재하는 일반 파일만 열어 수집 JSON을 기록합니다. symbolic link는 거부합니다. 부모 프로세스는 내용을 읽은 뒤 handoff 파일을 삭제합니다.

### 보안 취약점 제보

저장소가 Public으로 전환된 이후 보안 문제를 발견한 경우 공개 Issue에 민감한 재현정보를 바로 올리지 말고, Organization에서 제공하는 비공개 보안 제보 수단이 있다면 해당 수단을 우선 사용합니다. 별도 비공개 채널이 준비되지 않은 동안에는 민감정보가 포함된 로그/리포트를 공개 Issue에 첨부하지 마세요.

---

## English

HowAmI may use Administrator/root privileges to obtain detailed hardware information, so privilege elevation and external process execution are treated as security-sensitive code.

### Security principles

- HowAmI does not implement network upload or telemetry.
- When started by a normal user, final TXT/JSON output is written by the normal user process.
- The Administrator/root child performs collection and handoff writing only, then exits.
- Elevated code does not use arbitrary PATH-based executable discovery.
- Windows PowerShell is launched from `%SystemRoot%\\System32\\WindowsPowerShell\\v1.0\\powershell.exe`.
- macOS system tools use fixed `/usr/bin` or `/usr/sbin` locations.
- Linux helper tools are discovered only under `/usr/bin`, `/bin`, `/usr/sbin`, and `/sbin`.
- Collector subprocesses have timeouts.
- Values not exposed by the device/OS are not guessed.

### Temporary handoff file

The parent creates the handoff file before elevation and restricts it to mode `0600` on Unix-like systems. The elevated child opens only an already-existing regular file and rejects symbolic links. The parent reads the collection JSON and removes the handoff file afterward.

### Reporting a security issue

After the repository becomes public, prefer any private security-reporting channel provided by the Organization rather than posting sensitive reproduction details in a public Issue. Until a dedicated private channel is available, do not attach logs or reports containing personal identifiers to public Issues.
