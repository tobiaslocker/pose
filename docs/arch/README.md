# Architecture Documentation

This folder documents the current system architecture and key design decisions.
It loosely follows Arc42 and uses [C4 Model](https://c4model.com/) diagrams for visualization.

## Files

- [Context](./context.md): What the system is and how it fits into its environment
- [Components](./components.md): Main parts/modules and their responsibilities
- [Runtime](./runtime.md): Optional scenarios of dynamic behavior
- [Deployment](./deployment.md): Overview of deployment and communication setup
- [Decisions](./decisions/): Architecture Decision Records (ADRs)

---

> This documentation is maintained using the **docs-as-code** approach. Contributions welcome!

---

## Architecture Documentation Conventions

📅 2025-06-01

This document outlines the conventions used in this project to maintain a clean, lightweight, and
developer-friendly architecture documentation workflow.

---

### 🗂️ Project Structure

- All architecture documentation lives under:
  ```
  docs/arch/
  ```

- **Architecture Decisions** are stored in:
  ```
  docs/arch/decisions/
  ```

- ADR files follow a sequential format:
  ```
  NNN-title-of-decision.md
  ```

- An automatically generated `README.md` in `decisions/` serves as an **ADR log**.

---

### 📄 ADR File Format

Each ADR includes:

- A single H1 (`#`) heading for the title  
  → e.g. `# Use FlatBuffers for Pose Transport`

- Metadata at the top:
  ```markdown
  ![](https://img.shields.io/badge/status-accepted-brightgreen)

  ## Date

  📅 YYYY-MM-DD

  ```

- Content sections (recommended but flexible):
  - Context
  - Decision
  - Consequences

---

### 🪧 Status Conventions

Statuses are marked using a badge:

| Status   | Badge                                                                 |
|----------|-----------------------------------------------------------------------|
| Accepted | ![](https://img.shields.io/badge/status-accepted-brightgreen)         |
| Proposed | ![](https://img.shields.io/badge/status-proposed-yellow)              |
| Unknown  | ![](https://img.shields.io/badge/status-unknown-lightgrey)            |

Badge is inserted in the ADR log automatically based on `[Status: ...]` tag in the file.

---

### 🧾 ADR Log

The ADR log is auto-generated to `docs/arch/decisions/README.md` and includes:

- `Number` (parsed from filename)
- `Title` (linked)
- `Status` (badge)
- `Date` (from 📅 tag)

Column widths adjust to the longest entry per column.

---

### 🛠️ Tooling

- A script at `scripts/python/gen_adr_log.py` parses all ADRs and regenerates the log.
- It runs as part of:
  ```bash
  make docs
  ```

---

### 📖 Writing Guidelines

- Wrap lines at **100 characters**
- Prefer plain Markdown, avoid inline HTML
- Use **C4 model** where useful
- Use **PlantUML** to generate diagrams into:
  ```
  docs/arch/diagrams/
  ```

---

### 🧠 Workflow Integration

- New branches may be named after ADRs (e.g., `adr-005-websockets`)
- ADRs start as `[Status: Proposed]` and become `[Status: Accepted]` when implemented
- Git-based, lightweight process—no JIRA or issues required

---

### ✅ Example ADR Structure

```markdown
## Use FlatBuffers for Pose Data Serialization

![](https://img.shields.io/badge/status-accepted-brightgreen)

### Date

📅 2024-06-01  

### Context
Why the decision was needed.

### Decision
FlatBuffers over TCP was chosen due to performance and cross-language support.

### Consequences
+ Fast binary format
− May be harder to debug
```

