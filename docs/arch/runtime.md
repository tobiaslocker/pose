## Runtime Behavior

This section describes the runtime flow of the system from startup to interaction between 
components.

---

### Startup Flow

1. **Rust Server Initialization**  
   The server is launched (via `cargo run` or through `run.sh`) and opens a TCP socket on the 
   configured port (default: `9000`). It begins listening for incoming connections from pose input 
   sources.

2. **Pose Client Connection**  
   The Python-based pose client is started, typically from the same `run.sh` script. It establishes 
   a TCP connection to the server and begins streaming pose data in real time using MediaPipe and 
   the configured model.

3. **Data Transmission and Handling**  
   The client sends serialized pose frames (FlatBuffers) to the server at a regular frame rate. The 
   server:  
   - Receives each pose frame over the socket.  
   - Deserializes the FlatBuffer into internal Rust structs.  
   - Optionally logs or visualizes the data.  
   - Passes the data to the game logic pipeline (e.g., for scoring, visualization, etc.).

---

### Notes

- The server must be started **before** the client to ensure it can accept incoming connections.  
- Both components run independently but are orchestrated via the shared startup script.  
- The communication is one-directional: pose data is pushed from client to server in a 
  fire-and-forget manner.

### Runtime Diagram

![Runtime Flow](diagrams/runtime.svg)


