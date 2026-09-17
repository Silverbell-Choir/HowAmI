# Security / 보안

## 한국어

HowAmI는 하드웨어 정보를 최대한 읽기 위해 관리자/root 권한을 사용할 수 있으므로, 권한 상승 코드와 외부 명령 실행을 보수적으로 설계합니다.

### 보안 원칙

- HowAmI 자체에는 네트워크 업로드/텔레메트리 기능을 구현하지 않습니다.
- 일반 사용자로 실행하면 최종 TXT/JSON 저장은 일반 사용자 프로세스가 수행합니다.
- 관리자/root child는 시스템 정보 수집과 handoff 작성만 수행하고 종료합니다.
- elevated 상태에서 임의 PATH 검색을 사용하지 않습니다.
- Windows PowerShell은 Windows 시스템 디렉터리를 API로 확인한 뒤 그 아래 `WindowsPowerShell\\v1.0\\powershell.exe`를 사용합니다.
- macOS 시스템 도구는 고정된 `/usr/bin` 또는 `/usr/sbin` 경로를 사용합니다.
- Linux 보조 도구는 `/usr/bin`, `/bin`, `/usr/sbin`, `/sbin`에서만 찾습니다.
- 수집용 외부 명령에는 timeout을 적용합니다.
- 장치가 제공하지 않는 값은 추측하지 않습니다.

### 임시 handoff 파일

부모 프로세스가 먼저 고유한 임시 handoff 파일을 `create_new`로 생성하고, Unix 계열에서는 `0600` 권한으로 제한합니다. 파일에는 이번 권한 상승 흐름에 대응하는 handoff token을 먼저 기록합니다.

elevated child는 다음을 모두 확인한 뒤에만 파일을 덮어씁니다.

1. 실제 관리자/root 권한인지 확인
2. 전달된 경로가 이미 존재하는 일반 파일인지 확인
3. symbolic link가 아닌지 확인
4. 파일의 기존 token과 인자로 받은 token이 정확히 일치하는지 확인
5. HowAmI handoff marker 형식인지 확인

검증이 끝난 뒤에만 수집 JSON을 기록합니다. 부모 프로세스는 결과를 읽은 후 handoff 파일을 삭제합니다.

이 token은 암호학적 비밀키를 목적으로 하는 것이 아니라, 임의로 숨겨진 내부 인자만 호출하여 elevated HowAmI가 무관한 파일을 truncate하는 것을 방지하기 위한 handoff 인증 표식입니다.

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
- On Windows, the system directory is resolved through the Windows API and PowerShell is launched from its `WindowsPowerShell\\v1.0\\powershell.exe` child path.
- macOS system tools use fixed `/usr/bin` or `/usr/sbin` locations.
- Linux helper tools are discovered only under `/usr/bin`, `/bin`, `/usr/sbin`, and `/sbin`.
- Collector subprocesses have timeouts.
- Values not exposed by the device/OS are not guessed.

### Temporary handoff file

The parent creates a unique handoff file with `create_new` before elevation and restricts it to mode `0600` on Unix-like systems. It first writes a handoff token associated with that elevation flow.

Before overwriting the file, the elevated child verifies all of the following:

1. it is actually running with Administrator/root privileges,
2. the supplied path already refers to a regular file,
3. the path is not a symbolic link,
4. the token already stored in the file exactly matches the token supplied to the child,
5. the value uses the expected HowAmI handoff marker format.

Only then is collection JSON written. The parent reads the result and removes the handoff file afterward.

The token is not intended to be a cryptographic secret. Its purpose is to prevent someone from invoking only the hidden internal argument and causing an elevated HowAmI process to truncate an unrelated file.

### Reporting a security issue

After the repository becomes public, prefer any private security-reporting channel provided by the Organization rather than posting sensitive reproduction details in a public Issue. Until a dedicated private channel is available, do not attach logs or reports containing personal identifiers to public Issues.
