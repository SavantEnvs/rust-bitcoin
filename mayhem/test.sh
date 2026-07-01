#!/usr/bin/env bash
#
# mayhem/test.sh — RUN rust-bitcoin's OWN functional test suite (already built by
# mayhem/build.sh with the project's NORMAL flags). exit 0 = pass.
#
# Oracle: bitcoin-units' unit tests are known-answer / asserted-value tests (amount
# parsing/formatting, fee-rate, weight arithmetic, …). They assert BEHAVIOR/OUTPUT,
# so a no-op / neutered binary produces the wrong results and FAILS here
# (anti-reward-hack, SPEC §6.3). Emits a CTRF (ctrf.io) summary.
set -uo pipefail
[ -n "${SOURCE_DATE_EPOCH:-}" ] || unset SOURCE_DATE_EPOCH
: "${MAYHEM_JOBS:=$(nproc)}"
cd "${SRC:-/mayhem}"
SRC="${SRC:-/mayhem}"

emit_ctrf() {
  local tool="$1" passed="$2" failed="$3" skipped="${4:-0}" pending="${5:-0}" other="${6:-0}"
  local tests=$(( passed + failed + skipped + pending + other ))
  cat > "${CTRF_REPORT:-$SRC/ctrf-report.json}" <<JSON
{
  "results": {
    "tool": { "name": "$tool" },
    "summary": {
      "tests": $tests,
      "passed": $passed,
      "failed": $failed,
      "pending": $pending,
      "skipped": $skipped,
      "other": $other
    }
  }
}
JSON
  printf 'CTRF {"results":{"tool":{"name":"%s"},"summary":{"tests":%d,"passed":%d,"failed":%d,"pending":%d,"skipped":%d,"other":%d}}}\n' \
    "$tool" "$tests" "$passed" "$failed" "$pending" "$skipped" "$other"
  [ "$failed" -eq 0 ]
}

# RUN the pre-built suite (build.sh already compiled it; --no-fail-fast to see all
# results). Do NOT compile with the fuzz sanitizer flags — clear RUSTFLAGS so this
# stays the project's honest normal-flag oracle. cargo reuses build.sh's artifacts.
LOG="$(mktemp)"
env -u RUSTFLAGS cargo test -p bitcoin-units --locked --no-fail-fast -- --test-threads="$MAYHEM_JOBS" 2>&1 | tee "$LOG"
run_rc="${PIPESTATUS[0]}"

# cargo prints one "test result: ok. N passed; M failed; K ignored; ..." line per test
# binary (unit + doc). Sum them across all binaries for the aggregate counts.
passed=$(grep -oE 'test result: [a-zA-Z]+\. [0-9]+ passed' "$LOG" | grep -oE '[0-9]+ passed' | awk '{s+=$1} END{print s+0}')
failed=$(grep -oE '[0-9]+ failed' "$LOG" | awk '{s+=$1} END{print s+0}')
ignored=$(grep -oE '[0-9]+ ignored' "$LOG" | awk '{s+=$1} END{print s+0}')
had_results=0
grep -qE '^test result:' "$LOG" && had_results=1
rm -f "$LOG"

# Guard: if cargo itself failed (compile error / runner missing) but reported no test
# result lines, surface that as a failure rather than a spurious pass.
if [ "$run_rc" -ne 0 ] && [ "$failed" -eq 0 ] && [ "$passed" -eq 0 ]; then
  failed=1
fi
if [ "$had_results" -eq 0 ] && [ "$passed" -eq 0 ] && [ "$failed" -eq 0 ]; then
  echo "ERROR: no 'test result:' lines — test runner did not execute (build.sh bug or neutered binary)" >&2
  emit_ctrf "cargo-test-bitcoin-units" 0 1 0
  exit 1
fi

emit_ctrf "cargo-test-bitcoin-units" "$passed" "$failed" "$ignored"
