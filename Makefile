PROJECT_ROOT := $(realpath .)
PYTHON_DIR := $(PROJECT_ROOT)/python

FBS_FILE := $(PROJECT_ROOT)/schemas/pose.fbs
FBS_PY_OUTPUT := $(PROJECT_ROOT)/generated/python
FBS_RS_OUTPUT := $(PROJECT_ROOT)/generated/rust

DOCS_DIR := docs/arch/diagrams
PUML_FILES := $(wildcard $(DOCS_DIR)/*.puml)
SVG_FILES := $(PUML_FILES:.puml=.svg)

MODEL := $(PROJECT_ROOT)/models/pose_landmarker_lite.task
MODEL_URL := https://storage.googleapis.com/mediapipe-models/pose_landmarker/pose_landmarker_lite/float16/latest/pose_landmarker_lite.task

.PHONY: all flatbuffers python-deps clean rust-deps help fetch-models docs docs-clean

all: flatbuffers python-deps fetch-models

python-deps:
	@command -v poetry >/dev/null 2>&1 || { echo >&2 "Poetry is not installed. Please install Poetry (https://python-poetry.org/docs/#installation)."; exit 1; }
	@echo "Installing Python dependencies via Poetry..."
	@cd $(PYTHON_DIR) && poetry lock && poetry install

flatbuffers:
	@echo "Generating FlatBuffers for Python..."
	@mkdir -p $(FBS_PY_OUTPUT)
	@flatc --python -o $(FBS_PY_OUTPUT) $(FBS_FILE)
	@echo "Generating FlatBuffers for Rust..."
	@mkdir -p $(FBS_RS_OUTPUT)
	@flatc --rust --gen-all --gen-object-api -o $(FBS_RS_OUTPUT) $(FBS_FILE)

fetch-models:
	@echo "Fetching model: $(MODEL_URL)"
	@mkdir -p $(dir $(MODEL))
	@wget -q -O $(MODEL) $(MODEL_URL)
	@echo "Model downloaded to: $(MODEL)"

docs: $(SVG_FILES)
	@command -v plantuml >/dev/null 2>&1 || { echo >&2 "PlantUML is not installed. Please install it."; exit 1; }
	@echo "Generating ADR log..."
	@python3 scripts/python/gen_adr_log.py
	@echo "Documentation assets regenerated."

$(DOCS_DIR)/%.svg: $(DOCS_DIR)/%.puml
	@echo "[PLANTUML] Generating $@"
	@cd $(DOCS_DIR) && plantuml -tsvg $(notdir $<)

docs-clean:
	@echo "[PLANTUML] Cleaning generated diagrams..."
	@rm -f $(DOCS_DIR)/*.svg

rust-deps:
	@cargo build

clean:
	@rm -rf $(FBS_PY_OUTPUT)
	@rm -rf $(FBS_RS_OUTPUT)
	@rm -f $(MODEL)

help:
	@echo "Available make targets:"
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'

