## System Context

The system enables gesture- and movement-based interaction by processing live camera input to detect 
human poses in real time. These poses are matched against predefined motion sequences and visualized 
to provide immediate feedback to the user.

### Purpose and Scope

The system serves as a flexible core for various movement-based applications, including but not limited 
to:  
- Games  
- Fitness and exercise tracking  
- Motion-based rehabilitation  
- Interactive installations  

The architecture is designed to be modular and adaptable across platforms:  
- Desktop  
- Mobile (iOS/Android)  
- Browser-based (WASM)  
- Embedded systems  

### Key Concepts

- **Input:** Live video stream from a camera.  
- **Processing:** Detection of body landmarks from video frames.  
- **Matching:** Real-time comparison of live motion with predefined sequences.  
- **Output:** Visualization of poses and performance feedback.  

### Target Environments

#### Development

- Standard webcam for live capture.  
- Pose Detection Server and Visualization Client run locally.  
- Communication currently uses FlatBuffers over TCP.  

#### Production / Extended

- Pose Detection Server may run on a separate device (e.g. mobile phone, embedded).  
- Visualization Client can run standalone (desktop, browser, embedded).  
- Communication planned to move to WebSockets (WSS for secure connections).  
- Browser-based variants may run both detection and visualization client-side.  

### System Context Diagram

![System Context](diagrams/context.svg)

### Environment Notes

- The current prototype uses TCP/FlatBuffers with a Python-based Pose Detection Server and a 
  Bevy-based Visualization Client.  
- The client initiates the connection (game is client).  
- The architecture allows for future deployment flexibility and component substitution.  
