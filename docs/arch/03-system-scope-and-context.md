# 3. System Scope and Context

## 3.1 System Context

The system enables real-time and recorded motion visualization using pose landmarks. It supports streaming pose data from live detection or playback sources and renders skeletal representations in a game-like environment.

### 3.1.1 Purpose and Scope

The system provides:

- Live pose detection and visualization.
- Playback of recorded pose sequences.
- Real-time comparison and scoring of pose similarity.
- Audio-synchronized choreography visualization.

It is intended for experimentation with body-driven interaction, choreography tracking, and motion feedback.

### 3.1.2 Key Concepts

- **Live Stream:** Pose data streamed from a Python MediaPipe-based WebSocket server.
- **File Stream:** Pose data loaded from a prerecorded `.json` sequence.
- **PoseScore:** Per-frame comparison of live vs reference poses, independent of position and scale.
- **Visualization:** Landmarks and bones rendered using Bevy's 2D gizmos.

### 3.1.3 Architecture Principles

- Built with **Rust** and **Bevy**.
- **ECS-based**: Pose data is stored in components and updated by systems.
- Pluggable data sources implement the `LandmarkStream` abstraction.
- Designed for easy substitution of live vs file-based input.
- Lightweight, modular, and no unnecessary layers.

### 3.1.4 Deployment Scenarios

| Scenario             | Pose Source                  | Visualization             |
|----------------------|------------------------------|----------------------------|
| Desktop (current)    | Python + MediaPipe over WS   | Bevy native (Rust)        |
| Future (WASM)        | JS bridge or mobile stream    | Bevy WASM (browser)       |

## 5. Building Block View

### 5.1 Whitebox Overall System (Container View)

---

#### 🎮 Game Client (Rust + Bevy)

- Manages entities and systems via ECS.
- Receives pose data from either:
  - `ws_stream`: WebSocket live stream.
  - `file_stream`: Playback sequence.
- Renders landmark points and bone lines.
- Computes a `PoseScore` per frame.
- Displays score as a UI overlay.
- Synchronizes playback with audio.

---

#### 📡 Pose Detection Server (Python)

- Uses MediaPipe to detect pose landmarks.
- Streams FlatBuffers-encoded pose data over WebSocket.
- Can be started automatically by the game client.

---

### 5.2 Game Client Component View

#### Key Components and Systems

- **LandmarkFrame**: A single frame of pose data.
- **LatestLandmarkFrame**: Component storing the current pose per entity.
- **Playable / NonPlayable**: Tags for live vs reference characters.
- **PoseScore**: Component storing similarity score (only on live player).
- **LandmarkFrameReceiver**: Resource receiving WebSocket frames.
- **ScoreText**: UI entity displaying score.

#### Systems

- `file_stream`: Feeds frames from JSON sequence using audio time.
- `ws_stream`: Feeds live frames via WebSocket.
- `draw_character`: Renders all visible characters (landmarks and bones).
- `score_pose_similarity`: Compares live and reference poses per frame.
- `update_score_text`: Updates score overlay text each frame.
- `start_inference`: Starts the Python controller and waits for readiness.

---

### C4 Diagrams

#### System Context (C4 Level 1)

```plantuml
@startuml
!includeurl https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml

Person(user, "User", "Performs movements in front of webcam")

System(gameClient, "Game Client", "Rust + Bevy: Visualizes pose data and compares against reference")

System(poseServer, "Pose Detection Server", "Python + MediaPipe: Streams pose landmarks via WebSocket")

Rel(user, poseServer, "Moves in front of webcam")
Rel(poseServer, gameClient, "Streams pose landmarks (FlatBuffers over WS)")
Rel(gameClient, user, "Displays visual feedback and score")
@enduml
```

