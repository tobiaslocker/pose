# Pose Inference Server (Python)

This directory contains a modular, FlatBuffers-based TCP server designed to send pose detection data  
(e.g., from MediaPipe or dummy input) to a Rust client for real-time visualization and testing.

---

## 🚀 Setup

> This project uses [Poetry](https://python-poetry.org/) for dependency and environment management.

### 1. Install Poetry (recommended way)

Use the official Poetry installer — this ensures Poetry is isolated from system Python:

```bash
curl -sSL https://install.python-poetry.org | python3 -
```

Make sure your shell includes Poetry’s bin directory in your `PATH`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

You can add this line to your `~/.zshrc` or `~/.bashrc` for persistence.

---

### 2. Install Python 3.11+ via `pyenv` (recommended)

```bash
brew install pyenv openssl

env \
  LDFLAGS="-L$(brew --prefix openssl)/lib" \
  CPPFLAGS="-I$(brew --prefix openssl)/include" \
  PKG_CONFIG_PATH="$(brew --prefix openssl)/lib/pkgconfig" \
  pyenv install 3.11.9
```

---

### 3. Use `pyenv` Python in Poetry and enable in-project venv

```bash
cd python/

# Optional but recommended: keep venv inside project for portability
poetry config virtualenvs.in-project true

# Tell Poetry to use your pyenv Python version
poetry env use $(pyenv which python)

# Install dependencies
poetry install
```

---

## 🔧 Installing Dependencies

```bash
cd python/
poetry install
```

This sets up the `.venv/` virtual environment inside the project and installs all dependencies including `flatbuffers` and `pytest`.

---

## 🧪 Running Tests

To run all tests:

```bash
poetry run pytest
```

To run a specific test:

```bash
poetry run pytest tests/test_system_tcp.py
```

---

## 📡 FlatBuffers: Detection Schema

FlatBuffer-generated Python files live in:

```
../generated/python/Detection/
```

This is exposed to the project via `pyproject.toml`:

```toml
packages = [
  { include = "pose" },
  { include = "Detection", from = "../generated/python" }
]
```

Also configured in `pyrightconfig.json` for editor support:

```json
{
  "venvPath": ".",
  "venv": ".venv",
  "extraPaths": ["../generated/python"]
}
```

---

## 🧠 Development Notes

- `Server` is protocol-agnostic: it only sends/receives raw bytes over TCP.
- Length-prefixing and FlatBuffer encoding are handled at the application level.
- Dummy messages are used for system tests and prototyping.
- LSP (like Pyright) works with `pyrightconfig.json` and Poetry’s `.venv`.

---

## 🐍 Checking Your Environment

To verify that your setup is clean:

```bash
which poetry
# Should print: ~/.local/bin/poetry

poetry --version
# Should print: Poetry version X.X.X (no warnings)

poetry run python --version
# Should print: Python 3.11.x

poetry run python -m site
# Check that only .venv paths are listed; no ~/Library/Python/3.9 paths
```

---

