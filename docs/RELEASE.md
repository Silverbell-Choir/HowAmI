# Release / 릴리스

HowAmI는 GitHub Actions/유료 CI를 사용하지 않습니다. 릴리스는 대상 OS에서 로컬로 빌드하고 실제 장비에서 확인한 뒤 수동으로 게시합니다.

HowAmI does not use GitHub Actions or paid CI. Releases are built locally on each target OS, validated on real hardware, and published manually.

---

## 한국어

### 공개 전 게이트

저장소를 Public으로 전환하거나 첫 Release를 게시하기 전에 다음 조건을 충족합니다.

1. Windows 실제 장비에서 빌드/실행/리포트 검증
2. macOS 실제 장비에서 빌드/실행/리포트 검증
3. Linux 실제 장비에서 빌드/실행/리포트 검증
4. `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo build --release` 통과
5. 관리자/root 승인 및 거부 경로 확인
6. TXT/JSON 개인정보·고유 식별 정보 검토
7. `--output`과 `--no-elevate` 검증
8. 네트워크 통신/텔레메트리가 추가되지 않았는지 확인
9. Release에 사용할 `Cargo.lock` 커밋

### 권장 배포 파일명

```text
HowAmI-Windows-x64.exe
HowAmI-Windows-arm64.exe
HowAmI-macOS-x64
HowAmI-macOS-arm64
HowAmI-Linux-x64
HowAmI-Linux-arm64
SHA256SUMS.txt
```

각 파일은 해당 OS/아키텍처에서 직접 빌드한 산출물을 사용합니다. 다른 OS에서 크로스 컴파일한 파일은 실제 대상 장비 실행검증 없이 공식 Release로 게시하지 않습니다.

### 릴리스 절차

1. `Cargo.toml` 버전 확정
2. 대상 OS에서 로컬 검증 명령 실행
3. 실제 장비에서 HowAmI 실행
4. 생성된 TXT/JSON을 실제 하드웨어와 대조
5. 개인정보 검토
6. release binary 복사/이름 변경
7. SHA-256 체크섬 생성
8. Git tag 생성
9. GitHub Release에 binary + `SHA256SUMS.txt` 수동 업로드
10. Release notes를 한국어 + English로 작성

### 서명 관련

초기 무료 오픈소스 배포에서는 Windows 코드서명 인증서나 Apple Developer ID가 없을 수 있습니다. 이 경우 Windows SmartScreen/macOS Gatekeeper 경고가 발생할 수 있습니다. 경고를 우회하는 방법을 프로그램이 자동화하지 않습니다. 향후 공식 코드서명을 도입하면 이 문서를 갱신합니다.

---

## English

### Pre-publication gate

Before making the repository public or publishing the first Release, complete all of the following:

1. build/run/report validation on real Windows hardware,
2. build/run/report validation on real macOS hardware,
3. build/run/report validation on real Linux hardware,
4. pass `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `cargo build --release`,
5. verify both approval and denial paths for Administrator/root elevation,
6. review TXT/JSON output for personal and unique identifiers,
7. validate `--output` and `--no-elevate`,
8. confirm no network communication or telemetry was introduced,
9. commit the `Cargo.lock` used for the validated Release build.

### Recommended artifact names

```text
HowAmI-Windows-x64.exe
HowAmI-Windows-arm64.exe
HowAmI-macOS-x64
HowAmI-macOS-arm64
HowAmI-Linux-x64
HowAmI-Linux-arm64
SHA256SUMS.txt
```

Use artifacts built natively for the corresponding OS/architecture. Do not publish a cross-compiled artifact as an official Release without executing it on real target hardware first.

### Release procedure

1. finalize the version in `Cargo.toml`,
2. run local validation commands on each target OS,
3. execute HowAmI on real hardware,
4. compare TXT/JSON output with the actual machine,
5. complete privacy review,
6. copy/rename release binaries,
7. generate SHA-256 checksums,
8. create the Git tag,
9. manually upload binaries and `SHA256SUMS.txt` to the GitHub Release,
10. write Release notes in Korean + English.

### Signing

An early free/open-source release may not have a Windows code-signing certificate or Apple Developer ID. Windows SmartScreen or macOS Gatekeeper warnings may therefore appear. HowAmI does not automate bypassing those platform protections. Update this document if official signing is introduced later.
