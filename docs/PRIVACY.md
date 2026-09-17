# Privacy / 개인정보

## 한국어

HowAmI는 현재 **개인 사용자가 자기 컴퓨터를 상세 확인하는 용도**를 우선합니다. 따라서 상세 리포트에는 고유 식별 정보가 포함될 수 있습니다.

현재 수집될 수 있는 값:

- 메인보드/시스템/BIOS 관련 시리얼 및 UUID
- RAM 및 스토리지 시리얼 번호
- MAC 주소
- 컴퓨터 호스트명
- PnP/PCI/USB Instance ID 및 Device ID
- 드라이버/INF 식별 정보

HowAmI 코드에는 리포트 업로드, 원격 분석, 텔레메트리 기능을 넣지 않습니다. 생성 결과는 사용자가 지정한 로컬 경로 또는 기본 Desktop 경로에 저장됩니다.

### 공개 전 필수 검증

저장소를 Public으로 전환하기 전에 Windows/macOS/Linux 실제 장비에서 생성된 TXT/JSON을 검토하여 다음을 확정해야 합니다.

1. 기본 상세 리포트에 어떤 고유 식별 정보가 들어가는지
2. 향후 공유용 `safe-share` 모드가 필요한지
3. 사용자 계정명이나 홈 디렉터리 경로가 의도치 않게 출력되는지
4. OS별 명령의 Raw 출력에 예상하지 못한 개인정보가 포함되는지

---

## English

HowAmI currently prioritizes **detailed personal inspection of the user's own computer**. Detailed reports can therefore contain unique identifiers.

Values that may currently be collected include:

- motherboard/system/BIOS serials and UUIDs
- RAM and storage serial numbers
- MAC addresses
- computer host name
- PnP/PCI/USB instance and device IDs
- driver and INF identifiers

HowAmI does not implement report upload, remote analysis, or telemetry. Reports are written only to the user-selected local directory or the default Desktop location.

### Required review before making the repository public

Before switching the repository to Public, inspect real TXT/JSON reports from Windows, macOS, and Linux hardware and determine:

1. exactly which unique identifiers appear in the default detailed report,
2. whether a future `safe-share` mode is required,
3. whether user names or home-directory paths are exposed unintentionally,
4. whether any OS-specific raw command output contains unexpected personal data.
