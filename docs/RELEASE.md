# Release / 릴리스

이 문서는 HowAmI 배포 파일을 준비하고 GitHub Release로 게시할 때 사용하는 기준을 설명합니다.

This document describes the packaging and publication process for HowAmI releases.

---

## 한국어

### 배포 원칙

- 소스 저장소에는 소스코드, 문서, 스크립트, `Cargo.toml`, `Cargo.lock`만 유지합니다.
- `.exe`, ELF, macOS 실행파일 같은 **컴파일된 배포 바이너리는 소스 트리에 커밋하지 않습니다.**
- 사용자용 실행파일은 각 버전의 **GitHub Release Assets**로 제공합니다.
- 각 바이너리는 파일명과 일치하는 OS/아키텍처용으로 빌드합니다.
- 제공하지 않는 플랫폼에 빈 파일이나 다른 아키텍처의 복사본을 올리지 않습니다.

### 배포 파일명

```text
HowAmI-Windows-x64.exe
HowAmI-Windows-arm64.exe
HowAmI-macOS-x64
HowAmI-macOS-arm64
HowAmI-Linux-x64
HowAmI-Linux-arm64
SHA256SUMS.txt
BUILD-INFO.md
```

`BUILD-INFO.md`는 선택 사항이지만, 크로스빌드 또는 제한된 검증 범위를 가진 바이너리를 제공할 때는 포함하는 것을 권장합니다.

### 릴리스 준비

1. `Cargo.toml` 버전 확인
2. `Cargo.lock` 최신 상태 확인
3. 대상 플랫폼에서 release 빌드
4. 테스트 및 기본 실행 확인
5. TXT/JSON 생성 결과 확인
6. 배포 파일명 정리
7. SHA-256 체크섬 생성
8. 필요한 경우 `BUILD-INFO.md` 작성
9. Git tag 생성
10. GitHub Release 생성
11. 바이너리, 체크섬, 빌드 정보를 Release Assets로 업로드
12. 한국어 + English Release notes 작성

### 권장 검증

각 배포 바이너리에 대해 다음을 확인합니다.

- 대상 OS/아키텍처에서 실행되는지
- TXT와 JSON이 정상 생성되는지
- CPU/GPU/메인보드/BIOS/RAM/스토리지/모니터 등 주요 정보가 실제 장비와 일치하는지
- 관리자/root 권한 승인과 거부 흐름이 정상인지
- `--output`과 `--no-elevate`가 정상인지
- 개인정보·고유 식별 정보 안내가 실제 출력과 일치하는지
- SHA-256 체크섬이 배포 파일과 일치하는지

### BUILD-INFO.md 권장 항목

```text
Source commit
Build date
Rust version
Target triple
Artifact filename
Artifact size
Build method
Validation performed
Known limitations
Signing status
```

크로스빌드한 바이너리는 **빌드 성공**과 **대상 실장비 실행 확인**을 구분해 기록합니다.

### 체크섬

모든 배포 바이너리에 SHA-256 체크섬을 제공합니다.

```text
<sha256>  HowAmI-Windows-x64.exe
<sha256>  HowAmI-Linux-x64
```

### 코드 서명

서명되지 않은 바이너리는 Windows SmartScreen이나 macOS Gatekeeper 경고가 표시될 수 있습니다. 코드 서명을 적용하는 경우 Release notes 또는 `BUILD-INFO.md`에 서명 상태를 명시합니다.

---

## English

### Distribution policy

- Keep source code, documentation, scripts, `Cargo.toml`, and `Cargo.lock` in the source repository.
- **Do not commit compiled distribution binaries** such as `.exe`, ELF, or macOS executables to the source tree.
- Publish end-user binaries as **GitHub Release Assets** for each version.
- Build each binary for the OS and architecture named in its filename.
- Do not publish empty placeholders or copies from another architecture for unsupported targets.

### Artifact names

```text
HowAmI-Windows-x64.exe
HowAmI-Windows-arm64.exe
HowAmI-macOS-x64
HowAmI-macOS-arm64
HowAmI-Linux-x64
HowAmI-Linux-arm64
SHA256SUMS.txt
BUILD-INFO.md
```

`BUILD-INFO.md` is optional, but recommended for cross-built binaries or artifacts with a limited validation scope.

### Release preparation

1. confirm the version in `Cargo.toml`,
2. confirm `Cargo.lock` is current,
3. build release binaries for the target platforms,
4. run tests and a basic execution check,
5. verify TXT/JSON report generation,
6. normalize artifact filenames,
7. generate SHA-256 checksums,
8. create `BUILD-INFO.md` when useful,
9. create the Git tag,
10. create the GitHub Release,
11. upload binaries, checksums, and build information as Release Assets,
12. write Release notes in Korean + English.

### Recommended validation

For each distributed binary, verify:

- it runs on the target OS/architecture,
- TXT and JSON reports are generated correctly,
- major CPU/GPU/mainboard/BIOS/RAM/storage/display information matches the machine,
- Administrator/root approval and denial paths behave correctly,
- `--output` and `--no-elevate` work correctly,
- privacy/identifier documentation matches actual output,
- SHA-256 checksums match the distributed files.

### Recommended BUILD-INFO.md fields

```text
Source commit
Build date
Rust version
Target triple
Artifact filename
Artifact size
Build method
Validation performed
Known limitations
Signing status
```

For cross-built binaries, distinguish **successful compilation** from **execution on real target hardware**.

### Checksums

Provide SHA-256 checksums for all distributed binaries.

```text
<sha256>  HowAmI-Windows-x64.exe
<sha256>  HowAmI-Linux-x64
```

### Code signing

Unsigned binaries can trigger Windows SmartScreen or macOS Gatekeeper warnings. If signing is introduced, record the signing status in the Release notes or `BUILD-INFO.md`.
