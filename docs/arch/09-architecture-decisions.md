# 9. Architecture Decisions

This project maintains a structured Architecture Decision Log (ADR).

The log helps track key technical decisions over time and the rationale behind them.

## Current Decision Log

| Number | Title                                                                                                              | Status                                                                        | Date       |
|--------|--------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------|------------|
| 001    | [Use Bevy for Visualization and Game Logic](decisions/001-use-bevy.md)                                             | ![](https://img.shields.io/badge/status-accepted-brightgreen)                 | 2024-06-01 |
| 002    | [Use MediaPipe Pose Landmarker for Pose Detection](decisions/002-use-mediapipe.md)                                 | ![](https://img.shields.io/badge/status-accepted-brightgreen)                 | 2024-06-01 |
| 003    | [Use FlatBuffers over TCP for Pose Data Serialization and Transport](decisions/003-use-flatbuffers.md)             | ![](https://img.shields.io/badge/status-accepted-brightgreen)                 | 2024-06-01 |
| 004    | [Stick to 2D Rendering for Dance Visualization](decisions/004-stick-to-2d-rendering.md)                            | ![Status: Accepted](https://img.shields.io/badge/status-accepted-brightgreen) | 2024-06-01 |
| 005    | [Reverse Client-Server Roles and Use WebSockets](decisions/005-reverse-client-server-roles-and-use-web-sockets.md) | ![Status: Proposed](https://img.shields.io/badge/status-proposed-yellow)      | 2024-06-01 |

## Notes

- Proposed decisions are marked in yellow.
- This log is updated incrementally as new architecture decisions are made.

## Conventions

- Decisions are numbered sequentially (`001`, `002`, etc.).
- Each decision is stored as a separate `.md` file under `docs/arch/decisions/`.
- This section provides an index for quick navigation.
