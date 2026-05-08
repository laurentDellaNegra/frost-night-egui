#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"

usage() {
  cat <<EOF
Usage: ./run.sh <command>

Commands:
  check             Check the Rust workspace
  wasm-check        Check web crates for wasm32
  fmt               Check Rust formatting
  clippy            Run clippy for the workspace
  ci                Run the same Rust checks as CI
  demo              Run native demo
  demo-debug        Run native demo with debug assertions + backtraces
  tokens            Generate CSS tokens from Rust palette
  wasm-storybook    Build storybook WASM
  wasm-demo         Build demo WASM
  dev               Start Astro dev server
  site              Build Astro docs site
  preview           Preview Astro production build
  build             Full pipeline: tokens -> WASM -> Astro
  clean             Remove build artifacts
EOF
}

cmd_check() {
  echo "==> Checking Rust workspace..."
  (cd "$ROOT" && cargo check --workspace)
}

cmd_wasm_check() {
  echo "==> Checking web-demo for wasm32..."
  (cd "$ROOT" && cargo check -p web-demo --target wasm32-unknown-unknown)
  echo "==> Checking ui-storybook for wasm32..."
  (cd "$ROOT" && cargo check -p ui-storybook --target wasm32-unknown-unknown)
}

cmd_fmt() {
  echo "==> Checking Rust formatting..."
  (cd "$ROOT" && cargo fmt --all -- --check)
}

cmd_clippy() {
  echo "==> Running clippy..."
  (cd "$ROOT" && cargo clippy --workspace --all-targets --all-features -- -D warnings)
}

cmd_ci() {
  cmd_fmt
  echo "==> Checking frost-night-egui without default features..."
  (cd "$ROOT" && cargo check -p frost-night-egui --no-default-features)
  cmd_check
  echo "==> Checking frost-night-egui with all features..."
  (cd "$ROOT" && cargo check -p frost-night-egui --all-features)
  cmd_wasm_check
  cmd_clippy
}

cmd_demo() {
  (cd "$ROOT" && cargo run -p frost-night-demo)
}

cmd_demo_debug() {
  (cd "$ROOT" && RUST_BACKTRACE=1 RUST_LOG=warn cargo run -p frost-night-demo)
}

cmd_tokens() {
  echo "==> Generating tokens.css..."
  (cd "$ROOT" && cargo run -p frost-night-egui --example export_css 2>/dev/null) > "$ROOT/docs-site/src/styles/tokens.css"
  echo "    Done."
}

cmd_wasm_storybook() {
  echo "==> Building storybook WASM..."
  (cd "$ROOT/ui-storybook" && trunk build --release --public-url /frost-night-egui/wasm/ --dist "$ROOT/docs-site/public/wasm" --filehash false)
}

cmd_wasm_demo() {
  echo "==> Building demo WASM..."
  (cd "$ROOT/web-demo" && trunk build --release --public-url /frost-night-egui/demo/ --dist "$ROOT/docs-site/public/demo" --filehash false)
}

cmd_dev() {
  (cd "$ROOT/docs-site" && npm run dev)
}

cmd_site() {
  (cd "$ROOT/docs-site" && npm run build)
}

cmd_preview() {
  (cd "$ROOT/docs-site" && npm run preview)
}

cmd_build() {
  cmd_tokens
  cmd_wasm_storybook
  cmd_wasm_demo
  cmd_site
}

cmd_clean() {
  echo "==> Cleaning build artifacts..."
  rm -rf "$ROOT/target"
  rm -rf "$ROOT/docs-site/dist" "$ROOT/docs-site/.astro"
  rm -f "$ROOT/docs-site/public/wasm/"*.wasm "$ROOT/docs-site/public/wasm/"*.js
  rm -f "$ROOT/docs-site/public/demo/"*.wasm "$ROOT/docs-site/public/demo/"*.js
  echo "    Done."
}

case "${1:-}" in
  check)          cmd_check ;;
  wasm-check)     cmd_wasm_check ;;
  fmt)            cmd_fmt ;;
  clippy)         cmd_clippy ;;
  ci)             cmd_ci ;;
  demo)           cmd_demo ;;
  demo-debug)     cmd_demo_debug ;;
  tokens)         cmd_tokens ;;
  wasm-storybook) cmd_wasm_storybook ;;
  wasm-demo)      cmd_wasm_demo ;;
  dev)            cmd_dev ;;
  site)           cmd_site ;;
  preview)        cmd_preview ;;
  build)          cmd_build ;;
  clean)          cmd_clean ;;
  *)              usage ;;
esac
