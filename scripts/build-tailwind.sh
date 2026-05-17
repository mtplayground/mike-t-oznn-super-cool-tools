#!/usr/bin/env sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
BIN_DIR="$ROOT_DIR/.tailwindcss"
BIN_PATH="$BIN_DIR/tailwindcss"
INPUT_PATH="$ROOT_DIR/crates/toolbox-shell/styles/app.css"
OUTPUT_PATH="$ROOT_DIR/crates/toolbox-shell/dist/styles.css"
CONFIG_PATH="$ROOT_DIR/tailwind.config.js"

mkdir -p "$BIN_DIR" "$(dirname "$OUTPUT_PATH")"

if [ ! -x "$BIN_PATH" ]; then
  OS_NAME=$(uname -s)
  ARCH_NAME=$(uname -m)

  case "$OS_NAME" in
    Linux)
      PLATFORM="linux"
      ;;
    Darwin)
      PLATFORM="macos"
      ;;
    *)
      echo "unsupported operating system: $OS_NAME" >&2
      exit 1
      ;;
  esac

  case "$ARCH_NAME" in
    x86_64|amd64)
      ARCH="x64"
      ;;
    arm64|aarch64)
      ARCH="arm64"
      ;;
    *)
      echo "unsupported architecture: $ARCH_NAME" >&2
      exit 1
      ;;
  esac

  URL="https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.17/tailwindcss-$PLATFORM-$ARCH"
  curl --fail --location --silent --show-error --output "$BIN_PATH" "$URL"
  chmod +x "$BIN_PATH"
fi

"$BIN_PATH" \
  --config "$CONFIG_PATH" \
  --input "$INPUT_PATH" \
  --output "$OUTPUT_PATH"
