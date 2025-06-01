## Constraints

- **Client-Server Roles**
  - The controller must host a WebSocket server to stream pose data and receive scoring requests.
  - The game client connects as a WebSocket client to consume pose frames and send score requests.
  - Browser environments only allow outgoing WebSocket connections; no server sockets.

- **Browser Limitations**
  - WebAssembly (WASM) in browsers cannot host a TCP server socket.
  - Browser-based games must communicate exclusively via WebSockets (ws:// or wss://).

- **Performance Requirements**
  - Pose data streams at 30 FPS; serialization/deserialization must be zero-copy or near-zero-copy.
  - FlatBuffers is mandated as the wire format to minimize (de)serialization overhead.
  - Each FlatBuffers payload must fit within a single WebSocket binary frame to avoid fragmentation.

- **Platform Variety**
  - The controller may run on:
    - Python (desktop or embedded)
    - C++ (mobile or embedded)
    - Other embedded devices (e.g., Raspberry Pi)
  - The game client must support:
    - Rust + Bevy (native desktop)
    - WebAssembly (browser-only variant with MediaPipe JS inference)

- **Networking Environment**
  - Local firewalls or NAT may block arbitrary TCP ports.
  - WebSockets over HTTP(S) (ports 80/443) are more likely to traverse network barriers.
  - TLS (wss://) is required for browsers to avoid mixed-content issues.

- **Security & Deployment**
  - Browser clients must connect over WSS (TLS) in production.
  - Controllers must optionally support both ws:// (local development) and wss:// (production).
  - Firewall and permission requirements for binding ports on desktop, mobile, or embedded devices must
    be documented.

