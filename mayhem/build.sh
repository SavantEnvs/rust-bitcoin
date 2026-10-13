#!/usr/bin/env bash
#
# mayhem/build.sh — build rust-bitcoin's fuzz target(s) as sanitized libFuzzer
# binaries (OSS-Fuzz Rust path: cargo-fuzz + ASan via RUSTFLAGS), plus the
# project's own functional test suite (normal flags) so mayhem/test.sh can RUN it.
#
# Runs inside the commit image (RUST mayhem/Dockerfile) as `mayhem` in /mayhem.
# The Rust toolchain + cargo registry live at $CARGO_HOME=/opt/toolchains/rust/cargo
# (pinned by the Dockerfile ENV — absolute, $HOME-independent).
#
# AIR-GAPPED CONTRACT (SPEC §6.5): the PATCH tier re-runs THIS script OFFLINE.
#   - This FIRST build (in CI, online) populates the cargo registry under $CARGO_HOME.
#   - The PATCH re-run resolves crates from that cache. The rlenv runtime exports
#     CARGO_NET_OFFLINE=true for the re-run; we do NOT hard-code --offline here (it
#     would break this first, online build). Re-running on an already-built tree is
#     idempotent (cargo/cargo-fuzz skip up-to-date artifacts).
set -euo pipefail

# clang rejects SOURCE_DATE_EPOCH='' — must be unset or a valid integer.
[ -n "${SOURCE_DATE_EPOCH:-}" ] || unset SOURCE_DATE_EPOCH

: "${MAYHEM_JOBS:=$(nproc)}"
# cargo-fuzz has no --jobs flag; cargo reads parallelism from CARGO_BUILD_JOBS.
export CARGO_BUILD_JOBS="$MAYHEM_JOBS"

cd "${SRC:-/mayhem}"
SRC="${SRC:-/mayhem}"

# DWARF < 4 (§6.2 item 10): the fuzz binaries must carry DWARF <= 3 debug info so
# Mayhem's triage / ASan backtraces resolve source lines (Mayhem can't read DWARF >= 4).
# `-Zdwarf-version=3` is a nightly rustc flag (the image toolchain is nightly).
: "${RUST_DEBUG_FLAGS:=-Cdebuginfo=2 -Zdwarf-version=3 -Clinker=/opt/mayhem-dwarf3-anchor/cc-wrapper.sh}"

# Sanitizer selection. The base ENV defaults SANITIZER_FLAGS to ASan+UBSan (clang
# flags rustc ignores), so on the Rust path we translate its INTENT into RUSTFLAGS:
# if SANITIZER_FLAGS requests `address`, build with `-Zsanitizer=address` (the OSS-Fuzz
# Rust ASan path); an EXPLICIT empty SANITIZER_FLAGS (the off-switch, --build-arg
# SANITIZER_FLAGS=) builds a clean, un-instrumented binary. `=` (not `:=`) so an
# explicit empty value is preserved.
: "${SANITIZER_FLAGS=-fsanitize=address,undefined}"
RUST_SAN=""
CARGO_FUZZ_SAN="none"   # cargo-fuzz defaults to address; we drive it explicitly
case "$SANITIZER_FLAGS" in
  *address*) RUST_SAN="-Zsanitizer=address"; CARGO_FUZZ_SAN="address" ;;
esac

# OSS-Fuzz Rust libFuzzer flags. --cfg fuzzing matches libfuzzer-sys;
# force-frame-pointers aids ASan backtraces. Thread RUST_DEBUG_FLAGS for DWARF < 4.
export RUSTFLAGS="${RUSTFLAGS:-} --cfg fuzzing ${RUST_SAN} -Cforce-frame-pointers ${RUST_DEBUG_FLAGS}"

# Additive cargo-fuzz crate (leaves upstream's honggfuzz fuzz/ untouched).
FUZZ_DIR="mayhem/fuzz"
TRIPLE="x86_64-unknown-linux-gnu"

# Discover every target from the crate's fuzz_targets/ dir (one binary per target).
# The [[bin]] name is the basename for bitcoin/, prefixed for hashes/ and units/,
# matching mayhem/Cargo.toml. We derive the same names here.
FUZZ_TARGETS=()
for f in "$FUZZ_DIR"/fuzz_targets/*/*.rs; do
  sub="$(basename "$(dirname "$f")")"
  base="$(basename "${f%.*}")"
  if [ "$sub" = "bitcoin" ]; then name="$base"; else name="${sub}_${base}"; fi
  FUZZ_TARGETS+=("$name")
done
[ "${#FUZZ_TARGETS[@]}" -gt 0 ] || { echo "ERROR: no fuzz targets under $FUZZ_DIR/fuzz_targets/" >&2; exit 1; }

echo "=== cargo fuzz build (image nightly, ASan via RUSTFLAGS) ==="
echo "RUSTFLAGS=$RUSTFLAGS"
echo "targets (${#FUZZ_TARGETS[@]}): ${FUZZ_TARGETS[*]}"

# Use the image's DEFAULT toolchain (the Dockerfile pinned nightly). Build each
# target and copy its binary to /mayhem/<target> (matches mayhem/Mayhemfile_<target>).
for t in "${FUZZ_TARGETS[@]}"; do
  echo "--- building fuzz target: $t ---"
  cargo fuzz build --fuzz-dir "$FUZZ_DIR" --sanitizer "$CARGO_FUZZ_SAN" -O --debug-assertions "$t"
  bin="$SRC/$FUZZ_DIR/target/$TRIPLE/release/$t"
  [ -x "$bin" ] || { echo "ERROR: expected fuzz binary not found at $bin" >&2; exit 1; }
  cp "$bin" "/mayhem/$t"
  echo "built /mayhem/$t"
done

# ── Functional test suite (oracle for PATCH) ──────────────────────────────────
# Build the project's OWN tests with the project's NORMAL flags (a clean, NON-
# sanitized build — NOT the fuzz RUSTFLAGS) so mayhem/test.sh only RUNS them.
# We use bitcoin-units: it ships deterministic known-answer / asserted-value tests.
#
# The project keeps its pinned dependency solution in Cargo-recent.lock (Cargo.lock
# itself is .gitignore'd); materialize it as Cargo.lock so the test build resolves
# the SAME nightly-compatible deps every time (deterministic + air-gapped). Idempotent:
# a plain `cp -f` on an already-present lock is a no-op re-run.
[ -f Cargo-recent.lock ] && cp -f Cargo-recent.lock Cargo.lock
echo "=== building functional test suite (bitcoin-units, normal flags) ==="
env -u RUSTFLAGS cargo test --no-run -p bitcoin-units --locked

echo "build.sh complete"
