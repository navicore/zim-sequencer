#!/usr/bin/env bash
set -euo pipefail

ENGINE="./engine/target/debug/zim-sequencer"

echo "[TEST] Building zim-sequencer engine..."
cargo build --manifest-path=engine/Cargo.toml

echo "[TEST] Running smoke test..."
output=$(echo "C E G" | "$ENGINE")

if echo "$output" | grep -q "C major"; then
  echo "[PASS] Harmony analysis recognized C major triad."
else
  echo "[FAIL] Unexpected output:"
  echo "$output"
  exit 1
fi

