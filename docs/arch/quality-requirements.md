## Quality Requirements

- **Low Latency**  
  - Round-trip time for “pose_frame” → “score_request” → “score_update” should remain under 50 ms.  
  - Each FlatBuffers payload must fit in a single WebSocket binary frame to avoid fragmentation.  
  - End-to-end latency (camera capture → inference → rendering → feedback) should target < 30 ms in ideal conditions.

- **Cross-Platform Compatibility**  
  - A single FlatBuffers schema (`schemas/pose.fbs`) and WebSocket transport must work on:  
    - Rust (desktop Bevy client)  
    - Python or C++ (controller)  
    - JavaScript (browser-only WASM game via wasm-bindgen)  
  - Generated language bindings for FlatBuffers must remain consistent across all supported platforms.

- **Discoverability & Simplicity**  
  - The controller acts as a WebSocket server, eliminating the need for hard-coded IP/port pairs in clients.  
  - Documentation must clearly describe how to launch and connect the WebSocket server on each target device.  
  - Configuration (e.g., port numbers, TLS certificates) should be minimal and well-documented.

- **Maintainability & Evolvability**  
  - All FlatBuffers schema changes must be tracked in `schemas/pose.fbs`, using versioned namespaces if needed.  
  - Future protocol changes (e.g., new message types) should be backward-compatible or documented via ADRs.  
  - Code generation for FlatBuffers bindings must be automated in CI, ensuring schema and code stay in sync.

- **Security**  
  - Browser clients must connect over WSS (TLS) to avoid mixed-content issues.  
  - Controllers should support both unencrypted (ws://) for local development and encrypted (wss://) for production.  
  - TLS certificates and CORS rules must be documented, with clear instructions for obtaining and installing certs.

- **Reliability & Availability**  
  - The game client must retry WebSocket connections with exponential back-off (e.g., start at 1 s, double up to 10 s).  
  - Controllers should handle port-binding failures by logging errors and optionally retrying on a fallback port.  
  - In the event of schema mismatches (unrecognized `messageType`), frames should be dropped safely and warnings logged.

- **Scalability**  
  - While initial use is single-controller, single-client, the architecture should allow multiple controllers or clients in future.  
  - If multiple controllers become necessary, consider a message broker (e.g., NATS, MQTT) or a multiplexing strategy.  
  - The WebSocket server implementation should avoid blocking operations to support multiple concurrent connections.

