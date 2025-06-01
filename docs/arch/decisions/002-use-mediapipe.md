# Use MediaPipe Pose Landmarker for Pose Detection

![](https://img.shields.io/badge/status-accepted-brightgreen)

## Date

📅 2024-06-01

## Context

We require a reliable and accurate pose detection system to support the core gameplay of our  
camera-controlled dance game. The detector must:

- Work in real-time with acceptable latency
- Provide 2D and optionally 3D skeletal coordinates
- Be portable and integrable with Rust or via a language bridge
- Have a stable and well-documented model

### Alternatives Considered

#### 1. YOLOv8 + ONNX + OpenCV

- **Approach**: Use a pretrained YOLO model exported to ONNX, then load it with OpenCV DNN module  
  for inference.
- **Pros**:
  - Fully local and portable setup
  - Easy to integrate into Python or C++
- **Cons**:
  - General-purpose object detection, not optimized for pose landmarks
  - Limited pose fidelity (e.g. bounding boxes only or approximate joint estimation)
  - No built-in support for depth (z-axis) or confidence scores per joint

#### 2. MediaPipe Pose Landmarker

- **Approach**: Use Google's MediaPipe Pose Landmarker solution, available as `.task` files with  
  pretrained models and cross-platform runtime support.
- **Pros**:
  - Purpose-built for human pose estimation
  - Provides 33 high-precision landmarks with (x, y, z) coordinates and visibility scores
  - Actively maintained by Google and used in production
  - High accuracy and real-time performance
- **Cons**:
  - Installation and environment setup can be complex
  - Python or C++ API integration required (Rust interop handled via serialization)

## Decision

We chose **MediaPipe Pose Landmarker** as the pose detection engine for prototyping and development.

**Justifications**:
- High-quality 2D+3D landmark estimation with confidence scores
- Stable performance and superior accuracy versus general-purpose models
- Fits naturally with real-time applications like gaming or interactive feedback
- Easy prototyping in Python, with a future-ready C++ backend
- Well-supported and up-to-date by Google

## Consequences

- The `pose_landmarker_lite.task` model is used and downloaded automatically if missing
- Pose data is extracted in Python, serialized via FlatBuffers, and streamed to the Rust backend
- Future versions may migrate to a native C++ implementation for performance
- Python dependencies (e.g., `mediapipe`) are installed in a dedicated virtual environment

