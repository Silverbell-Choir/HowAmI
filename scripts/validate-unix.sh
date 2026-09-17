#!/usr/bin/env sh
set -eu

printf '%s\n' 'HowAmI local validation / 로컬 검증'

if ! command -v cargo >/dev/null 2>&1; then
  printf '%s\n' 'Rust/Cargo is not installed or not available in PATH.' >&2
  exit 1
fi

printf '%s\n' '[1/4] cargo fmt --check'
cargo fmt --check

printf '%s\n' '[2/4] cargo test --locked'
cargo test --locked

printf '%s\n' '[3/4] cargo clippy --all-targets --locked -- -D warnings'
cargo clippy --all-targets --locked -- -D warnings

printf '%s\n' '[4/4] cargo build --release --locked'
cargo build --release --locked

printf '%s\n' 'Validation completed / 검증 완료: target/release/HowAmI'
printf '%s\n' 'Run the binary and review both generated TXT and JSON before making the repository public.'
printf '%s\n' '실행 후 생성된 TXT/JSON을 확인한 다음 Public 전환 여부를 결정하세요.'
