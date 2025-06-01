## Deployment View

### Local Development Setup

The application is designed for local development and prototyping using the following setup:

- **Python Virtual Environment**: Python 3.12 environment with dependencies managed via `venv`  
  and `requirements.txt`.
- **Rust Backend**: Built and run with `cargo`. Exposes a TCP server to receive serialized pose  
  frames.
- **FlatBuffers**: Used for defining and generating shared data structures between components.
- **Orchestration**: A single `scripts/run.sh` script sets up the environment, builds dependencies,  
  generates required code, and runs both server and client with coordinated startup.
- **Communication**: Local TCP socket (default port `9000`) connects the client and server.

### Execution

Run the system locally via:

```bash
./scripts/run.sh
```

### Deployment Diagram

![Deployment](diagrams/deployment.svg?v=2)
