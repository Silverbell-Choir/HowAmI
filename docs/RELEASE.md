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
4. `cargo fmt --check`, `cargo test --locked`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo build --release --locked` 통과
5. 관리자/root 승인 및 거부 경로 확인
6. TXT/JSON 개인정보·고유 식별 정보 검토
7. `--output`과 `--no-elevate` 검증
8. 네트워크 통신/텔레메트리가 추가되지 않았는지 확인
9. Release에 사용할 `Cargo.lock` 커밋

### v0.1.0 준비 상태 (2026-09-17)

- 준비됨: Windows x64 네이티브 빌드, Linux x64/arm64 정적 musl 크로스빌드
- 실행 검증됨: Windows x64 `--no-elevate` 경로와 TXT/JSON 구조
- 실행 미검증: Linux x64/arm64 실장비
- 생성하지 않음: Windows arm64(ARM64 MSVC/CRT 링크 라이브러리 없음), macOS x64/arm64(Apple SDK·링커·실장비 없음)

준비된 파일은 `release/dist/v0.1.0/`에 저장합니다. 이 디렉터리의 세 실행 파일과 `SHA256SUMS.txt`를 그대로 Release 자산으로 사용할 수 있지만, 위 공개 전 게이트가 완료되기 전에는 공식 Release로 게시하지 않습니다.

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

각 파일은 해당 OS/아키텍처에서 직접 빌드한 산출물을 우선 사용합니다. 안전하게 크로스빌드한 후보는 빌드 방식과 미검증 상태를 `BUILD-INFO.md`에 명시하고, 실제 대상 장비 실행검증 없이 공식 Release로 게시하지 않습니다. 파일이 없는 대상에는 빈 파일이나 다른 아키텍처의 복사본을 대신 올리지 않습니다.

### 릴리스 절차

1. `Cargo.toml` 버전 확정
2. 대상 OS에서 로컬 검증 명령 실행
3. 실제 장비에서 HowAmI 실행
4. 생성된 TXT/JSON을 실제 하드웨어와 대조
5. 개인정보 검토
6. release binary 복사/이름 변경
7. SHA-256 체크섬 생성
8. Git tag 생성
9. `release/dist/<version>/`의 binary + `SHA256SUMS.txt` + `BUILD-INFO.md`를 GitHub Release에 수동 업로드
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
4. pass `cargo fmt --check`, `cargo test --locked`, `cargo clippy --all-targets --locked -- -D warnings`, and `cargo build --release --locked`,
5. verify both approval and denial paths for Administrator/root elevation,
6. review TXT/JSON output for personal and unique identifiers,
7. validate `--output` and `--no-elevate`,
8. confirm no network communication or telemetry was introduced,
9. commit the `Cargo.lock` used for the validated Release build.

### v0.1.0 preparation status (2026-09-17)

- Prepared: native Windows x64 build and static musl Linux x64/arm64 cross-builds
- Executed: Windows x64 `--no-elevate` path with TXT/JSON structure validation
- Not executed: Linux x64/arm64 on target hardware
- Not produced: Windows arm64 (missing ARM64 MSVC/CRT link libraries) and macOS x64/arm64 (no Apple SDK, linker, or target hardware)

Prepared files are stored in `release/dist/v0.1.0/`. Its three executables and `SHA256SUMS.txt` are ready to be used as Release assets, but must not be published as an official Release until the pre-publication gate above is complete.

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

Prefer artifacts built natively for the corresponding OS/architecture. For safely cross-built candidates, record the build method and unvalidated status in `BUILD-INFO.md`; do not publish them as an official Release without executing them on real target hardware. Never substitute an empty file or a copy from another architecture for a missing target.

### Release procedure

1. finalize the version in `Cargo.toml`,
2. run local validation commands on each target OS,
3. execute HowAmI on real hardware,
4. compare TXT/JSON output with the actual machine,
5. complete privacy review,
6. copy/rename release binaries,
7. generate SHA-256 checksums,
8. create the Git tag,
9. manually upload the binaries, `SHA256SUMS.txt`, and `BUILD-INFO.md` from `release/dist/<version>/` to the GitHub Release,
10. write Release notes in Korean + English.

### Signing

An early free/open-source release may not have a Windows code-signing certificate or Apple Developer ID. Windows SmartScreen or macOS Gatekeeper warnings may therefore appear. HowAmI does not automate bypassing those platform protections. Update this document if official signing is introduced later.
