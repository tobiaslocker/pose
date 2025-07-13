import asyncio
import argparse
import websockets
import os
import json
import time
import cv2
import mediapipe as mp
import asyncio
from mediapipe.tasks import python
from mediapipe.tasks.python import vision
from mediapipe.tasks.python.vision.pose_landmarker import PoseLandmarkerResult
from dataclasses import dataclass
import numpy as np
import mediapipe as mp
from mediapipe.framework.formats import landmark_pb2
from enum import Enum
import asyncio
import websockets
import flatbuffers
import Detection.DetectionMessage as DetectionMessage
import Detection.DetectionPayload as DetectionPayload
import Detection.PoseDetectionResult as PoseDetectionResult
import Detection.Landmark as Landmark
import Detection.Availability as Availability
import cv2
import mediapipe as mp
import subprocess

mp_drawing = mp.solutions.drawing_utils # type: ignore[attr-defined]
mp_pose = mp.solutions.pose # type: ignore[attr-defined]

def draw_result(result):
    if not result.result.pose_landmarks:
        return

    colors = [
        (0, 255, 0),
        (255, 0, 0),
        (0, 0, 255),
    ]

    for idx, pose_landmarks in enumerate(result.result.pose_landmarks):
        color = colors[idx % len(colors)]
        landmark_proto = landmark_pb2.NormalizedLandmarkList() # type: ignore[attr-defined]
        for lm in pose_landmarks:
            landmark_proto.landmark.add(
                x=lm.x,
                y=lm.y,
                z=lm.z,
                visibility=lm.visibility
            )

        mp_drawing.draw_landmarks(
            result.frame,
            landmark_proto,
            mp_pose.POSE_CONNECTIONS,
            mp_drawing.DrawingSpec(color=color, thickness=2, circle_radius=2),
            mp_drawing.DrawingSpec(color=color, thickness=2)
        )

    cv2.imshow("MediaPipe Drawing", result.frame)
    cv2.waitKey(1)


def parse(pose_landmarker_result) -> bytes:
    """Serialize PoseLandmarkerResult to FlatBuffers DetectionMessage (as bytes)."""

    builder = flatbuffers.Builder(1024)
    landmark_offsets = []

    if len(pose_landmarker_result.result.pose_landmarks) == 0:
        DetectionMessage.Start(builder)
        DetectionMessage.AddPayloadType(builder, DetectionPayload.DetectionPayload().Empty)
        detection_message_offset = DetectionMessage.End(builder)
        builder.Finish(detection_message_offset)
        return bytes(builder.Output())

    for landmark in pose_landmarker_result.result.pose_landmarks[0]:
        Availability.Start(builder)
        Availability.AddVisibility(builder, landmark.visibility)
        Availability.AddPresence(builder, landmark.presence)
        availability_offset = Availability.End(builder)

        Landmark.Start(builder)
        Landmark.AddX(builder, landmark.x)
        Landmark.AddY(builder, landmark.y)
        Landmark.AddZ(builder, landmark.z)
        Landmark.AddAvailability(builder, availability_offset)
        landmark_offset = Landmark.End(builder)
        landmark_offsets.append(landmark_offset)

    PoseDetectionResult.StartLandmarksVector(builder, len(landmark_offsets))
    for lm_offset in reversed(landmark_offsets):
        builder.PrependUOffsetTRelative(lm_offset)
    landmarks_vector = builder.EndVector()

    PoseDetectionResult.Start(builder)
    PoseDetectionResult.AddLandmarks(builder, landmarks_vector)
    PoseDetectionResult.AddTimestamp(builder, pose_landmarker_result.timestamp)
    pose_detection_result_offset = PoseDetectionResult.End(builder)

    DetectionMessage.Start(builder)
    DetectionMessage.AddPayloadType(builder, DetectionPayload.DetectionPayload().PoseDetectionResult)
    DetectionMessage.AddPayload(builder, pose_detection_result_offset)
    detection_message_offset = DetectionMessage.End(builder)
    builder.Finish(detection_message_offset)

    return bytes(builder.Output())


async def forward(source, queue, parser):
    async for item in source:
        parsed = parser(item)
        if parsed is None:
            continue
        await queue.put(parsed)


class Server:
    def __init__(self, host='0.0.0.0', port=9000, queue=asyncio.Queue()):
        self.host = host
        self.port = port
        self.queue = queue

    async def handle_client(self, websocket, _):
        print("WebSocket client connected")
        try:
            while True:
                msg = await self.queue.get()
                if msg is None:
                    print("Send loop received shutdown signal.")
                    break
                await websocket.send(msg)
        except websockets.ConnectionClosed:
            print("WebSocket client disconnected")

    async def run(self):
        async with websockets.serve(
            self.handle_client,
            self.host,
            self.port,
        ):
            print(f"[WebSocket] Serving on ws://{self.host}:{self.port}")
            await asyncio.Future()


@dataclass
class Result:
    timestamp: float
    result: PoseLandmarkerResult
    frame: np.ndarray
    success: bool


class PoseLandmarkDetection:

    def __init__(self, video_capture, model_asset_path, flip):
        base_options = python.BaseOptions(model_asset_path=model_asset_path)

        def save_result(result, output_image, timestamp_ms):
            _ = output_image
            _ = timestamp_ms
            self.latest_result = result

        options = vision.PoseLandmarkerOptions(
            base_options=base_options,
            running_mode=vision.RunningMode.LIVE_STREAM,
            num_poses=1,
            min_pose_detection_confidence=0.5,
            min_pose_presence_confidence=0.5,
            min_tracking_confidence=0.5,
            result_callback=save_result
        )

        self.cap = video_capture
        self.frame_idx = 0
        self.fps = self.cap.get(cv2.CAP_PROP_FPS)
        self.detector = vision.PoseLandmarker.create_from_options(options)
        self.latest_result = None
        self.flip = flip

    @classmethod
    def fromfilename(cls, filename, model_asset_path):
        cap = cv2.VideoCapture(filename)
        if not cap.isOpened():
            raise RuntimeError(f"Failed to open video file: {filename}")
        return cls(cap, model_asset_path, False)

    @classmethod
    def fromindex(cls, index,  model_asset_path, width=1280, height=960):
        cap = cv2.VideoCapture(index)
        cap.set(cv2.CAP_PROP_FRAME_WIDTH, width)
        cap.set(cv2.CAP_PROP_FRAME_HEIGHT, height)
        return cls(cap, model_asset_path, True)

    async def next(self):
        success, frame = self.cap.read()
        if not success:
            return None

        rgb_image = None

        if self.flip:
            frame = cv2.flip(frame, 1)

        rgb_image = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
        mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=rgb_image)
        self.detector.detect_async(mp_image, time.time_ns() // 1_000_000)

        while self.latest_result is None:
            await asyncio.sleep(0.005)


        timestamp = (self.frame_idx / self.fps) * 1000.0
        #print(self.frame_idx, self.fps, timestamp)
        self.frame_idx += 1

        result = self.latest_result
        self.latest_result = None
        return Result(timestamp, result, frame, success)

    def to_generator(self):
        async def generator():
            while True:
                result = await self.next()
                if result is None:
                    break
                yield result

        return generator()


def capture_frame(timestamp, landmarks):
    return {
        "timestamp": timestamp,
        "landmarks": [
            {
                "x": lm.x,
                "y": lm.y,
                "z": lm.z,
                "availability": {
                  "visibility": lm.visibility,
                  "presence": lm.presence
                }
            }
            for lm in landmarks
        ]
    }


async def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--model", required=True)
    parser.add_argument("--show-preview", action="store_true")
    parser.add_argument("--input-video", help="Path to input video file instead of webcam")
    args = parser.parse_args()

    if args.input_video:
        inference = PoseLandmarkDetection.fromfilename(args.input_video, args.model)
    else:
        inference = PoseLandmarkDetection.fromindex(0, args.model)

    source = inference.to_generator()

    async def with_preview(source):
        async for result in source:
            if result.success:
                draw_result(result)
            yield result

    if args.show_preview:
        source = with_preview(source)

    if args.input_video:
        recording = { "name": f"performer", "frames": [] }
        async for result in source:
            if result.success:
                for landmarks in result.result.pose_landmarks:
                    frame_data = capture_frame(result.timestamp, landmarks)
                    recording["frames"].append(frame_data)

        out_dir = "recordings"
        os.makedirs(out_dir, exist_ok=True)
        filename = os.path.join(out_dir, f"performer_1.json")
        with open(filename, "w") as f:
            json.dump(recording, f, indent=2)

        subprocess.run([
            "ffmpeg",
            "-i", args.input_video,
            "-vn",
            "-acodec", "libvorbis",
            os.path.join(out_dir, "output.ogg")
        ], check=True)

    else:
        queue = asyncio.Queue(maxsize=32)
        server = Server(host="127.0.0.1", port=9000, queue=queue)

        server_runner = await websockets.serve(
            server.handle_client, server.host, server.port, ping_interval=None
        )

        forward_task = asyncio.create_task(forward(source, queue, parse))
        await forward_task

        server_runner.close()
        await server_runner.wait_closed()

        try:
            await server.run()
        except asyncio.CancelledError:
            pass


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("Server stopped cleanly.")

