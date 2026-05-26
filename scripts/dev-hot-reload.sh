#!/usr/bin/env bash
set -euo pipefail

cargo run --package content --features dev-tools --bin content-watch &
watch_pid=$!

cargo leptos watch --project frontend &
leptos_pid=$!

cleanup() {
  kill "$watch_pid" "$leptos_pid" >/dev/null 2>&1 || true
}

trap cleanup EXIT INT TERM

wait -n "$watch_pid" "$leptos_pid"
status=$?

echo "A dev process exited (status $status). Shutting down..."
exit "$status"
