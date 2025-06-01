## Component Overview

The system is organized into the following main components:

- **Pose Capture Client**  
  Acquires pose landmarks using a MediaPipe-based model, performs real-time inference, and 
  serializes the data using FlatBuffers before streaming it to the server.

- **Pose Serializer**  
  Converts structured landmark data into a FlatBuffer binary format for transmission.

- **Server Core**  
  Listens for pose data from the client, receives and handles binary messages.

- **Pose Deserializer**  
  Converts received FlatBuffer messages into native Rust data structures for further use.

- **Pose Matcher / Score Evaluator**  
  Compares live pose input against pre-recorded choreography and computes a similarity score 
  to determine how closely the player's movements align with the target dance moves.

- **Visualization Engine (Bevy)**  
  Displays both live and prerecorded poses side-by-side in a game-like 3D environment, helping 
  players visually align their motions.

- **Content Recorder**  
  Captures and stores prerecorded choreography using the same pose detection system as the 
  live client, enabling the creation of game content from real performances.

### Component Diagram

The following diagram provides a high-level view of the core components and their interactions in the system:

![Component Diagram](diagrams/components.svg)

