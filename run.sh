#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"

usage() {
  cat <<EOF
Usage: ./run.sh <command>

Commands:
  check             Check all Rust crates
  demo              Run native demo
  tokens            Generate CSS tokens from Rust palette
  wasm-storybook    Build storybook WASM
  wasm-demo         Build demo WASM
  dev               Start Astro dev server
  site              Build Astro docs site
  preview           Preview Astro production build
  build             Full pipeline: tokens → WASM → Astro
  clean             Remove build artifacts
EOF
}

cmd_check() {
  echo "==> Checking ui-theme..."
  (cd "$ROOT/ui-theme" && cargo check)
  echo "==> Checking ui-storybook..."
  (cd "$ROOT/ui-storybook" && cargo check)
}

cmd_demo() {
  (cd "$ROOT/ui-theme" && cargo run --example demo --features demo)
}

cmd_tokens() {
  echo "==> Generating tokens.css..."
  (cd "$ROOT/ui-theme" && cargo run --example export_css 2>/dev/null) > "$ROOT/docs-site/src/styles/tokens.css"
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
  rm -rf "$ROOT/ui-theme/target" "$ROOT/ui-storybook/target" "$ROOT/web-demo/target"
  rm -rf "$ROOT/docs-site/dist" "$ROOT/docs-site/.astro"
  rm -f "$ROOT/docs-site/public/wasm/"*.wasm "$ROOT/docs-site/public/wasm/"*.js
  rm -f "$ROOT/docs-site/public/demo/"*.wasm "$ROOT/docs-site/public/demo/"*.js
  echo "    Done."
}

case "${1:-}" in
  check)          cmd_check ;;
  demo)           cmd_demo ;;
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
