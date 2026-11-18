#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
flutter_rust_bridge_codegen generate
pattern='^[[:space:]]*(//|/\*|\*([[:space:]]|/|$))'
targets=$(find lib/src/rust hook -name '*.dart'; echo rust/src/frb_generated.rs)
for file in $targets; do
  grep -vE "$pattern" "$file" > "$file.stripped" || true
  mv "$file.stripped" "$file"
done
dart format lib hook
cargo fmt -p pulse_mobile_bridge
