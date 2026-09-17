# Release / 릴리스

이 문서는 HowAmI 배포 파일을 준비하고 GitHub Release로 게시할 때 사용하는 기준을 설명합니다.

This document describes the packaging and publication process for HowAmI releases.

---

## 한국어

### 배포 파일명

```text
HowAmI-Windows-x64.exe
HowAmI-Windows-arm64.exe
HowAmI-macOS-x64
HowAmI-macOS-arm64
HowAmI-Linux-x64
HowAmI-Linux-arm64
SHA256SUMS.txt
```

각 바이너리는 파일명과 일치하는 OS/아키텍처용으로 빌드해야 합니다. 다른 아키텍처의 파일을 복사하거나 빈 파일을 대신 배포하지 않습니다.

### 릴리스 준비

1. `Cargo.toml` 버전 확인
2. `Cargo.lock` 최신 상태 확인
3. 대상 플랫폼에서 release 빌드
4. 테스트 및 기본 실행 확인
5. TXT/JSON 생성 결과 확인
6. 바이너리 파일명 정리
7. SHA-256 체크섬 생성
8. `BUILD-INFO.md`에 대상, 빌드 도구, 검증 범위 기록
9. Git tag 생성
10. GitHub Release에 바이너리와 체크섬 업로드
11. 한국어 + English Release notes 작성

### 권장 검증

각 배포 바이너리에 대해 다음을 확인합니다.

- 프로그램이 대상 OS/아키텍처에서 실행되는지
- TXT와 JSON이 정상 생성되는지
- CPU/GPU/메인보드/BIOS/RAM/스토리지/모니터 등 주요 정보가 실제 장비와 일치하는지
- 관리자/root 권한 승인과 거부 흐름이 정상인지
- `--output`과 `--no-elevate`가 정상인지
- 개인정보·고유 식별 정보 안내가 실제 출력과 일치하는지
- SHA-256 체크섬이 배포 파일과 일치하는지

### BUILD-INFO.md

각 버전의 `BUILD-INFO.md`에는 다음 정보를 기록합니다.

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
```

크로스빌드한 바이너리는 크로스빌드 여부와 실장비 실행 여부를 구분해 표시합니다.

### 체크섬

모든 배포 바이너리에 SHA-256 체크섬을 제공합니다.

예:

```text
<sha256>  HowAmI-Windows-x64.exe
<sha256>  HowAmI-Linux-x64
```

### 코드 서명

Windows 코드서명 또는 Apple Developer ID 서명을 사용하지 않은 바이너리는 Windows SmartScreen이나 macOS Gatekeeper 경고가 표시될 수 있습니다.

서명을 적용하는 경우 `BUILD-INFO.md`와 Release notes에 서명 여부를 명시합니다.

---

## English

### Artifact names

```text
HowAmI-Windows-x64.exe
HowAmI-Windows-arm64.exe
HowAmI-macOS-x64
HowAmI-macOS-arm64
HowAmI-Linux-x64
HowAmI-Linux-arm64
SHA256SUMS.txt
```

Each binary must be built for the operating system and architecture named in its filename. Do not substitute files from another architecture or publish empty placeholders.

### Release preparation

1. confirm the version in `Cargo.toml`,
2. confirm `Cargo.lock` is current,
3. build release binaries for target platforms,
4. run tests and a basic execution check,
5. verify TXT/JSON report generation,
6. normalize artifact filenames,
7. generate SHA-256 checksums,
8. record target, toolchain, and validation details in `BUILD-INFO.md`,
9. create the Git tag,
10. upload binaries and checksums to the GitHub Release,
11. write Release notes in Korean + English.

### Recommended validation

For each distributed binary, verify:

- it runs on the target OS/architecture,
- TXT and JSON reports are generated correctly,
- major CPU/GPU/mainboard/BIOS/RAM/storage/display information matches the machine,
- Administrator/root approval and denial paths behave correctly,
- `--output` and `--no-elevate` work correctly,
- privacy/identifier documentation matches actual output,
- SHA-256 checksums match the distributed files.

### BUILD-INFO.md

For each version, `BUILD-INFO.md` should record:

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
```

Cross-built binaries should clearly distinguish cross-compilation from execution on real target hardware.

### Checksums

Provide SHA-256 checksums for all distributed binaries.

Example:

```text
<sha256>  HowAmI-Windows-x64.exe
<sha256>  HowAmI-Linux-x64
```

### Code signing

Unsigned binaries can trigger Windows SmartScreen or macOS Gatekeeper warnings.

If code signing is introduced, record the signing status in `BUILD-INFO.md` and the Release notes.
