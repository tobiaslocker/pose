# 3. System Scope and Context

## 3.1 System Context

The system compares and visualizes human pose landmarks from two sources: live input via WebSocket and prerecorded sequences from a file. It renders both with Bevy and computes a normalized similarity score per frame.

### 3.1.1 Purpose and Scope

The system supports:

- Live pose input via MediaPipe streamed over WebSocket.
- Playback of recorded pose sequences synchronized with audio.
- Visualization of pose landmarks and skeletal structure.
- Real-time scoring of similarity between live and recorded pose frames.

### 3.1.2 Key Concepts

- **Live Pose:** Frame data received from an external Python-based pose server.
- **Reference Pose:** Frame data loaded from a JSON sequence.
- **PoseScore:** A normalized score comparing live and reference poses.
- **LandmarkFrame:** Timestamped pose data used across systems.
- **Visualization:** Bevy-based rendering using gizmos for points and lines.

### 3.1.3 Architecture Principles

- Implemented in Rust using Bevy ECS and Kira audio.
- WebSocket frame reception runs in a separate Tokio thread.
- Python pose server is spawned by the game client.
- No abstraction layers or traits — direct system-to-component updates.
- Pose comparison is position- and scale-invariant.

### 3.1.4 Current and Planned Deployment Scenarios

| Scenario             | Pose Source                  | Visualization Client     |
|----------------------|-------------------------------|---------------------------|
| Desktop              | Python Pose Server (WebSocket) | Bevy Native (Rust)        |
| Web (planned)        | MediaPipe via JS/WebRTC       | Bevy WASM                 |
| Mobile Controller    | (future)                      | Bevy Native/WASM          |

### 3.1.5 C4 Level 1 Diagram (System Context Diagram)

![C4 Level 1 Diagram](diagrams/c4-level-1-system-context.svg)

### 3.1.6 Environment Notes

- Live pose frames are streamed using FlatBuffers over WebSocket.
- The `Playable` entity represents the live user.
- The `NonPlayable` entity plays the prerecorded sequence.
- Audio timing controls playback synchronization.
- All pose logic is in ECS systems without intermediate traits or plugins.

---

## 5. Building Block View

### 5.1 Whitebox Overall System (Container View)

---

#### 🎮 Game Client (Rust + Bevy)

- Starts the Python server with `start_inference()`.
- Reads prerecorded sequence from JSON into `Sequence` resource.
- Spawns a WebSocket listener to stream pose frames into a `Playable` entity.
- Spawns a `NonPlayable` entity to play the reference sequence using audio timing.
- Compares pose landmarks and calculates similarity in `score_pose_similarity`.
- Renders points and bones for both poses using gizmos.
- Displays the similarity score with `Text` UI.

---

#### 📡 Pose Detection Server (Python)

- Uses MediaPipe to detect pose landmarks from webcam input.
- Streams pose frames via WebSocket as FlatBuffers.
- Started from within the game client and auto-terminated on exit.

---

### 5.2 Game Client Component View

#### Key Components

- `LatestLandmarkFrame`: Holds the current frame for an entity.
- `Playable`: Marks the live input entity.
- `NonPlayable`: Marks the reference (playback) entity.
- `PoseScore(f32)`: Component holding a similarity score.
- `ScoreText`: Marks a `Text` UI displaying the score.
- `Sequence`: Resource holding the landmark sequence as `VecDeque<LandmarkFrame>`.
- `LandmarkFrameReceiver`: Receiver for incoming WebSocket pose frames.
- `AudioHandle`: Reference to the currently playing audio instance.

#### Key Systems

- `setup`: Spawns UI, camera, and landmark entities.
- `file_stream`: Feeds the `NonPlayable` entity frames from file based on audio playback time.
- `ws_stream`: Updates `Playable` with latest live pose from WebSocket.
- `draw_character`: Renders points and bones per entity using gizmos.
- `score_pose_similarity`: Computes score by normalizing and comparing poses.
- `update_score_text`: Displays the current score as a `Text` overlay.

---

### System Container Diagram (C4 Level 2)

![System Container Diagram](diagrams/c4-level-2-system-container.svg)

---

### 5.2 Game Client Component Diagram (C4 Level 3)

![Container Component Diagram: Game Client](diagrams/c4-level-3-component-game-client.svg)

