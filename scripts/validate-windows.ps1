$ErrorActionPreference = 'Stop'

Write-Host 'HowAmI local validation / 로컬 검증'

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw 'Rust/Cargo is not installed or not available in PATH.'
}

Write-Host '[1/4] cargo fmt --check'
cargo fmt --check
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host '[2/4] cargo test'
cargo test
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host '[3/4] cargo clippy --all-targets -- -D warnings'
cargo clippy --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host '[4/4] cargo build --release'
cargo build --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$binary = Join-Path $PSScriptRoot '..\target\release\HowAmI.exe'
$binary = [System.IO.Path]::GetFullPath($binary)
Write-Host "Validation completed / 검증 완료: $binary"
Write-Host 'Run the binary and review both generated TXT and JSON before making the repository public.'
Write-Host '실행 후 생성된 TXT/JSON을 확인한 다음 Public 전환 여부를 결정하세요.'
