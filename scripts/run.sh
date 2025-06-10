#!/bin/bash

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/.."
cat "$PROJECT_ROOT/scripts/ascii/pose_art.txt"

SERVER_PORT=9000
VENV_PATH="$PROJECT_ROOT/scripts/python/venv"
FBS_OUTPUT_PATH="$PROJECT_ROOT/generated"
MODEL_PATH="$PROJECT_ROOT/models/pose_landmarker_lite.task"
PY_SERVER_PATH="$PROJECT_ROOT/python/pose/run_server.py"

SETUP_PREFIX="\033[1;35m[POSE SETUP]\033[0m"
SERVER_PREFIX="\033[1;32m[POSE SERVER]\033[0m"
CLIENT_PREFIX="\033[1;36m[POSE CLIENT]\033[0m"
MAKE_PREFIX="\033[1;33m[POSE MAKE]\033[0m"

FORCE=false

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

run_prefixed() {
    prefix="$1"
    shift
    "$@" 2>&1 | stdbuf -oL sed "s/^/$(echo -e "$prefix ")/"
}

cleanup() {
    echo -e "${SETUP_PREFIX} Stopping Python server..."
    kill $SERVER_PID 2>/dev/null || true
    exit
}
trap cleanup INT TERM

if ! command -v python3.12 >/dev/null; then
    echo -e "${SETUP_PREFIX} Python 3.12 not found. Please install it (or use pyenv)."
    exit 1
fi

if ! command -v make >/dev/null; then
    echo -e "${SETUP_PREFIX} Error: make is not installed."
    exit 1
fi

if [ "$FORCE" = true ]; then
    echo -e "${SETUP_PREFIX} Forcing rebuild of all components..."
    run_prefixed "${MAKE_PREFIX}" make -C "$PROJECT_ROOT" python-deps flatbuffers fetch-models
else
    echo -e "${SETUP_PREFIX} Ensuring Python dependencies are installed..."
    run_prefixed "${MAKE_PREFIX}" make -C "$PROJECT_ROOT" python-deps
    if [ ! -d "$FBS_OUTPUT_PATH" ] || [ -z "$(ls -A "$FBS_OUTPUT_PATH")" ]; then
        echo -e "${SETUP_PREFIX} FlatBuffers output missing. Generating via make..."
        run_prefixed "${MAKE_PREFIX}" make -C "$PROJECT_ROOT" flatbuffers
    fi

    if [ ! -f "$MODEL_PATH" ]; then
        echo -e "${SETUP_PREFIX} Pose model missing. Fetching via make..."
        run_prefixed "${MAKE_PREFIX}" make -C "$PROJECT_ROOT" fetch-models
    fi
fi

echo -e "${SETUP_PREFIX} Starting Python server..."
poetry run --directory "${PROJECT_ROOT}/python" python "$PY_SERVER_PATH" --model "$MODEL_PATH" \
  > >(stdbuf -oL sed "s/^/$(echo -e "${SERVER_PREFIX} ")/") 2>&1 &
SERVER_PID=$!

echo -e "${SETUP_PREFIX} Waiting for Python server on port $SERVER_PORT..."
until nc -z localhost $SERVER_PORT; do
    sleep 0.2
done
echo -e "${SETUP_PREFIX} Server is ready."

run_prefixed "${CLIENT_PREFIX}" cargo run

wait $SERVER_PID
