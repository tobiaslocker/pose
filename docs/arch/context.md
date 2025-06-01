## System Context

The purpose of this project is to develop a game controlled entirely by a camera. Inspired by 
games like "Just Dance", the core idea is to offer an engaging dance-based experience where 
players try to mirror a prerecorded choreography as accurately as possible.

The choreography is captured in advance using the same pose-detection system as the player. Both 
recorded and live poses are visualized using Bevy, providing real-time feedback. The game then 
compares the live player input against the recorded content to compute a matching score, which 
can be used for scoring, progression, or feedback.

While dance is the primary use case, the system is designed to be extensible to other movement-
based experiences such as boxing, fitness routines, or motion-based rehabilitation. The platform 
is still to be determined, but the focus is on modularizing motion input, real-time detection, 
and motion matching to enable flexibility across multiple front-ends or devices.

### System Context Diagram
![System Context](diagrams/context.svg)

### Environment

This system is intended to run locally during development, using:

- A standard webcam for pose capture.
- A desktop or laptop capable of running MediaPipe and Bevy in parallel.
- Local TCP communication between the pose client and the Rust game server.

For production or extended setups, hardware placement (e.g., camera height), lighting, and 
platform capabilities may influence system behavior and performance.

