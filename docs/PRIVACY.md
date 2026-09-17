# Privacy / 개인정보

## 한국어

HowAmI는 현재 **개인 사용자가 자기 컴퓨터를 상세 확인하는 용도**를 우선합니다. 따라서 기본 상세 리포트에는 고유 식별 정보가 포함될 수 있습니다.

현재 포함될 수 있는 값:

- 메인보드/시스템/BIOS 관련 시리얼 및 UUID
- RAM/스토리지/모니터/USB 장치 시리얼 번호
- MAC 주소
- 컴퓨터 호스트명
- PnP/PCI/USB Instance ID 및 Device ID
- 볼륨/파일시스템 UUID
- 입력 장치 Unique ID
- 드라이버/INF 식별 정보
- Linux/macOS 시스템 도구가 반환하는 기타 장치 식별자

HowAmI 코드에는 리포트 업로드, 원격 분석, 텔레메트리 기능을 넣지 않습니다. 생성 결과는 사용자가 지정한 로컬 경로 또는 기본 Desktop 경로에 저장됩니다.

### 관리자/root 권한과 파일 소유권

일반 사용자로 실행한 경우 최종 TXT/JSON 파일은 일반 사용자 부모 프로세스가 생성합니다. 관리자/root child는 수집 결과만 임시 handoff 파일로 반환하고 종료합니다. 따라서 macOS/Linux에서 최종 리포트가 의도치 않게 root 소유가 되는 것을 피하도록 설계되어 있습니다.

### 공개 전 필수 검증

저장소를 Public으로 전환하기 전에 Windows/macOS/Linux 실제 장비에서 생성된 TXT/JSON을 검토하여 다음을 확정합니다.

1. 기본 상세 리포트에 실제로 어떤 고유 식별 정보가 들어가는지
2. 사용자 계정명이나 홈 디렉터리 경로가 의도치 않게 출력되는지
3. Linux의 `lspci`/`lsusb`/`dmidecode`, macOS 시스템 도구 등 Raw/flattened 출력에 예상하지 못한 개인정보가 포함되는지
4. 볼륨/파일시스템 UUID, 입력 장치 Unique ID 등 일반 사용자가 개인정보로 인식하지 못할 수 있는 식별자가 어떻게 표시되는지
5. README의 개인정보 안내가 실제 출력과 일치하는지
6. 향후 공유용 `--safe-share` 모드가 필요한지

현재 목적은 **개인용 상세 확인**이므로 식별 정보를 자동으로 숨기지 않습니다. 외부 공유 전에는 반드시 사용자가 리포트 내용을 직접 확인해야 합니다.

---

## English

HowAmI currently prioritizes **detailed personal inspection of the user's own computer**. The default detailed report can therefore contain unique identifiers.

Values that may appear include:

- motherboard/system/BIOS serials and UUIDs
- RAM/storage/monitor/USB serial numbers
- MAC addresses
- computer host name
- PnP/PCI/USB instance and device IDs
- volume/filesystem UUIDs
- input-device unique IDs
- driver/INF identifiers
- other identifiers returned by Linux/macOS system tools

HowAmI does not implement report upload, remote analysis, or telemetry. Reports are written only to the user-selected local directory or the default Desktop location.

### Administrator/root access and file ownership

When started by a normal user, the final TXT/JSON files are created by the normal user parent process. The Administrator/root child returns collection data through a temporary handoff file and exits. This is designed to avoid unintentionally creating final reports owned by root on macOS/Linux.

### Required review before making the repository public

Before switching the repository to Public, inspect real TXT/JSON reports from Windows, macOS, and Linux hardware and determine:

1. exactly which unique identifiers appear in the default detailed report,
2. whether account names or home-directory paths are exposed unintentionally,
3. whether raw/flattened output from tools such as Linux `lspci`/`lsusb`/`dmidecode` or macOS system tools contains unexpected personal data,
4. how identifiers such as volume/filesystem UUIDs and input-device unique IDs are presented,
5. whether README privacy guidance matches actual output,
6. whether a future `--safe-share` profile is desirable.

The current goal is **detailed personal inspection**, so identifiers are not automatically redacted. Users should review the report before sharing it externally.
