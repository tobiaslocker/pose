#!/bin/bash

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/.."
cat "$PROJECT_ROOT/scripts/ascii/pose_art.txt"

SERVER_PORT=9000
VENV_PATH="$PROJECT_ROOT/scripts/python/venv"
FBS_OUTPUT_PATH="$PROJECT_ROOT/generated"
MODEL_PATH="$PROJECT_ROOT/models/pose_landmarker_lite.task"
PY_SCRIPT_PATH="$PROJECT_ROOT/scripts/python/client.py"

# Define prefixes
SETUP_PREFIX="\033[1;35m[POSE SETUP]\033[0m"
SERVER_PREFIX="\033[1;32m[POSE SERVER]\033[0m"
CLIENT_PREFIX="\033[1;36m[POSE CLIENT]\033[0m"
MAKE_PREFIX="\033[1;33m[POSE MAKE]\033[0m"

FORCE=false

# Parse args
for arg in "$@"; do
  case $arg in
    --force)
      FORCE=true
      shift
      ;;
    *)
      echo -e "${SETUP_PREFIX} Unknown argument: $arg"
      exit 1
      ;;
  esac
done

# Prefixing function
run_with_prefix() {
    name="$1"
    shift
    stdbuf -oL "$@" 2>&1 | awk -v prefix="$name" '{ print prefix " " $0 }'
}

# Cleanup on Ctrl+C or error
cleanup() {
    echo -e "${SETUP_PREFIX} Stopping Rust server..."
    kill $SERVER_PID 2>/dev/null || true
    exit
}
trap cleanup INT TERM

# Check required tools
if ! command -v python3.12 >/dev/null; then
    echo -e "${SETUP_PREFIX} Python 3.12 not found. Please install it (or use pyenv)."
    exit 1
fi

if ! command -v make >/dev/null; then
    echo -e "${SETUP_PREFIX} Error: make is not installed."
    exit 1
fi

# Build via Makefile depending on force flag
if [ "$FORCE" = true ]; then
    echo -e "${SETUP_PREFIX} Forcing rebuild of all components..."
    run_with_prefix "${MAKE_PREFIX}" make -C "$PROJECT_ROOT" python-deps flatbuffers fetch-models
else
    if [ ! -d "$VENV_PATH" ]; then
        echo -e "${SETUP_PREFIX} Virtual environment not found. Creating it via make..."
        run_with_prefix "${MAKE_PREFIX}" make -C "$PROJECT_ROOT" python-deps
    fi

    if [ ! -d "$FBS_OUTPUT_PATH" ] || [ -z "$(ls -A "$FBS_OUTPUT_PATH")" ]; then
        echo -e "${SETUP_PREFIX} FlatBuffers output missing. Generating via make..."
        run_with_prefix "${MAKE_PREFIX}" make -C "$PROJECT_ROOT" flatbuffers
    fi

    if [ ! -f "$MODEL_PATH" ]; then
        echo -e "${SETUP_PREFIX} Pose model missing. Fetching via make..."
        run_with_prefix "${MAKE_PREFIX}" make -C "$PROJECT_ROOT" fetch-models
    fi
fi

# Activate venv
source "$VENV_PATH/bin/activate"

# Start Rust server in background
run_with_prefix "${CLIENT_PREFIX}" cargo run &
SERVER_PID=$!

# Wait for the server to become ready
echo -e "${SETUP_PREFIX} Waiting for Rust server on port $SERVER_PORT..."
until nc -z localhost $SERVER_PORT; do
    sleep 0.2
done
echo -e "${SETUP_PREFIX} Server is ready."

# Run Python client
run_with_prefix "${CLIENT_PREFIX}" python3 "$PY_SCRIPT_PATH" --model "$MODEL_PATH"

# Cleanup
kill $SERVER_PID

