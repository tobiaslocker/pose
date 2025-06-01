# Makefile
PROJECT_ROOT := $(realpath .)
VENV_DIR := $(PROJECT_ROOT)/scripts/python/venv
PYTHON := python3.12
REQUIREMENTS := $(PROJECT_ROOT)/scripts/python/requirements.txt

FBS_FILE := $(PROJECT_ROOT)/schemas/pose.fbs
FBS_PY_OUTPUT := $(PROJECT_ROOT)/generated/python
FBS_RS_OUTPUT := $(PROJECT_ROOT)/generated/rust

DOCS_DIR := docs/arch/diagrams
PUML_FILES := $(wildcard $(DOCS_DIR)/*.puml)
SVG_FILES := $(PUML_FILES:.puml=.svg)


MODEL := $(PROJECT_ROOT)/models/pose_landmarker_lite.task
MODEL_URL := https://storage.googleapis.com/mediapipe-models/pose_landmarker/pose_landmarker_lite/float16/latest/pose_landmarker_lite.task

.PHONY: all flatbuffers python-deps venv run clean rust-deps help fetch-models docs

## Generate all documentation assets (e.g. diagrams)
docs:
	@echo "Generating ADR log..."
	@python3 scripts/python/gen_adr_log.py
	@echo "Documentation assets regenerated."


$(DOCS_DIR)/%.svg: $(DOCS_DIR)/%.puml
	@echo "[PLANTUML] Generating $@"
	@plantuml -tsvg -o . $<

## Clean generated documentation assets
docs-clean:
	@echo "[PLANTUML] Cleaning generated diagrams..."
	@rm -f $(DOCS_DIR)/*.svg
## Build everything
all: flatbuffers python-deps fetch-models

## Always recreate Python venv
venv:
	@echo "Recreating Python venv..."
	@rm -rf $(VENV_DIR)
	@$(PYTHON) -m venv $(VENV_DIR)

## Always reinstall Python dependencies
python-deps: venv
	@echo "Installing Python dependencies..."
	@$(VENV_DIR)/bin/pip install -r $(REQUIREMENTS)

## Always regenerate FlatBuffers for Python and Rust
flatbuffers:
	@echo "Generating FlatBuffers for Python..."
	@flatc --python -o $(FBS_PY_OUTPUT) $(FBS_FILE)
	@echo "Generating FlatBuffers for Rust..."
	@mkdir -p $(FBS_RS_OUTPUT)
	@flatc --rust -o $(FBS_RS_OUTPUT) $(FBS_FILE)

## Always download the MediaPipe model
fetch-models:
	@echo "Fetching model: $(MODEL_URL)"
	@mkdir -p $(dir $(MODEL))
	@wget -q -O $(MODEL) $(MODEL_URL)
	@echo "Model downloaded to: $(MODEL)"

## Run full stack via run.sh
run:
	@./scripts/run.sh

## Optional Rust dependency build
rust-deps:
	@cargo build

## Clean all generated assets
clean:
	@rm -rf $(FBS_PY_OUTPUT)
	@rm -rf $(FBS_RS_OUTPUT)
	@rm -rf $(VENV_DIR)
	@rm -f $(MODEL)

## Show help text
help:
	@echo "Available make targets:"
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'

