import argparse
import mediapipe as mp
from mediapipe.tasks import python
from mediapipe.tasks.python import vision
from mediapipe.framework.formats import landmark_pb2
import cv2

mp_drawing = mp.solutions.drawing_utils # type: ignore[attr-defined]
mp_pose = mp.solutions.pose # type: ignore[attr-defined]

def draw_result(result_list, frame):
    colors = [
        (0, 255, 0),
        (255, 0, 0),
        (0, 0, 255),
    ]

    for i, result in enumerate(result_list):
        landmark_proto = landmark_pb2.NormalizedLandmarkList() # type: ignore[attr-defined]
        for lm in result:
            landmark_proto.landmark.add(
                x=lm.x,
                y=lm.y,
                z=lm.z,
                visibility=lm.visibility
            )

        mp_drawing.draw_landmarks(
            frame,
            landmark_proto,
            mp_pose.POSE_CONNECTIONS,
            mp_drawing.DrawingSpec(color=colors[i % len(colors)], thickness=2, circle_radius=2),
            mp_drawing.DrawingSpec(color=colors[i % len(colors)], thickness=2)
        )

    cv2.imshow("MediaPipe Drawing", frame)
    cv2.waitKey(1)


def run_record(args):
    import os
    import json
    import subprocess
    from pathlib import Path

    cap = cv2.VideoCapture(args.input_video)
    if not cap.isOpened():
        raise RuntimeError(f"Failed to open video input: {args.input_video}")

    base_options = python.BaseOptions(model_asset_path=args.model)
    options = vision.PoseLandmarkerOptions(
        base_options=base_options,
        running_mode=vision.RunningMode.VIDEO,
        num_poses=args.num_poses,
        min_pose_detection_confidence=0.5,
        min_pose_presence_confidence=0.5,
        min_tracking_confidence=0.5,
    )
    detector = vision.PoseLandmarker.create_from_options(options)

    recordings = [
        { "fps": cap.get(cv2.CAP_PROP_FPS), "frames": [] } for _ in range(args.num_poses)
    ]

    while True:
        success, frame = cap.read()
        if not success:
            break

        rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
        mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=rgb)

        result = detector.detect_for_video(
            mp_image,
            int((cap.get(cv2.CAP_PROP_POS_FRAMES) / cap.get(cv2.CAP_PROP_FPS)) * 1000.0),
        )
        for i in range(args.num_poses):
            sorted_people = []
            
            if result.pose_landmarks:
                sorted_people = sorted(result.pose_landmarks, key=lambda lms: lms[0].x)
                if args.show_preview:
                    draw_result(sorted_people, frame)

            if i < len(sorted_people):
                recordings[i]["frames"].append({
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
                        for lm in sorted_people[i]
                    ]
                })
            else:
                recordings[i]["frames"].append({ "landmarks": [] })

    os.makedirs(args.output_dir, exist_ok=True)
    for i in range(args.num_poses):
        json_path = os.path.join(args.output_dir, Path(args.input_video).stem + f"_{i}.json")
        with open(json_path, "w") as f:
            json.dump(recordings[i], f, indent=2)

        print(f"[record] Saved landmarks to {json_path}")

    audio_path = os.path.join(args.output_dir, Path(args.input_video).stem + ".ogg")
    subprocess.run([
        "ffmpeg",
        "-i", args.input_video,
        "-vn",
        "-acodec", "libvorbis",
        audio_path
    ], check=True)

    print(f"[record] Extracted audio to {audio_path}")


def run_serve(args):
    import threading
    import asyncio
    import websockets
    import queue
    import flatbuffers
    import Detection.DetectionMessage as DetectionMessage
    import Detection.DetectionPayload as DetectionPayload
    import Detection.PoseDetectionResult as PoseDetectionResult
    import Detection.Landmark as Landmark
    import Detection.Availability as Availability

    result_queue = queue.Queue()
    latest_result = None

    def result_callback(result, output_image, timestamp_ms):
        _ = timestamp_ms
        _ = output_image
        nonlocal latest_result
        latest_result = result
        if result.pose_landmarks:
            result_queue.put_nowait(result)

    cap = cv2.VideoCapture(args.input_device)
    cap.set(cv2.CAP_PROP_FRAME_WIDTH, args.width)
    cap.set(cv2.CAP_PROP_FRAME_HEIGHT, args.height)

    base_options = python.BaseOptions(model_asset_path=args.model)

    options = vision.PoseLandmarkerOptions(
        base_options=base_options,
        running_mode=vision.RunningMode.LIVE_STREAM,
        num_poses=1,
        min_pose_detection_confidence=0.5,
        min_pose_presence_confidence=0.5,
        min_tracking_confidence=0.5,
        result_callback=result_callback
    )
    detector = vision.PoseLandmarker.create_from_options(options)
    frame_index = 0

    async def handle_connection(websocket, path):
        _ = path
        print("[serve] Client connected")
        try:
            while True:
                try:
                    result = result_queue.get(timeout=1.0)
                    if result:
                        builder = flatbuffers.Builder(1024)
                        landmark_offsets = []

                        if len(result.pose_landmarks) == 0:
                            DetectionMessage.Start(builder)
                            DetectionMessage.AddPayloadType(builder, DetectionPayload.DetectionPayload().Empty)
                            detection_message_offset = DetectionMessage.End(builder)
                            builder.Finish(detection_message_offset)
                            return bytes(builder.Output())

                        for landmark in result.pose_landmarks[0]:
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
                        pose_detection_result_offset = PoseDetectionResult.End(builder)

                        DetectionMessage.Start(builder)
                        DetectionMessage.AddPayloadType(builder, DetectionPayload.DetectionPayload().PoseDetectionResult)
                        DetectionMessage.AddPayload(builder, pose_detection_result_offset)
                        detection_message_offset = DetectionMessage.End(builder)
                        builder.Finish(detection_message_offset)
                        await websocket.send(bytes(builder.Output()))

                except queue.Empty:
                    continue
        except websockets.ConnectionClosed:
            print("[serve] Client disconnected")

    async def run_ws_server():
        server = await websockets.serve(handle_connection, args.ip, args.port)
        await server.wait_closed()

    threading.Thread(target=lambda: asyncio.run(run_ws_server()), daemon=True).start()

    try:
        while True:
            success, frame = cap.read()
            if args.flip:
                frame = cv2.flip(frame, 1)
            if not success:
                break

            rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
            mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=rgb)

            timestamp = int((frame_index / cap.get(cv2.CAP_PROP_FPS)) * 1000.0)
            frame_index += 1
            detector.detect_async(mp_image, timestamp)

            if args.show_preview and latest_result and latest_result.pose_landmarks:
                draw_result(latest_result.pose_landmarks, frame)

    except KeyboardInterrupt:
        print("\n[serve] Stopped by user.")
    finally:
        cap.release()
        cv2.destroyAllWindows()


def main():
    parser = argparse.ArgumentParser(prog="pose_tool")
    subparsers = parser.add_subparsers(dest="command", required=True)

    # Recording subcommand
    record_parser = subparsers.add_parser("record", help="Extract poses from a video and save as JSON.")
    record_parser.add_argument("--input-video", required=True)
    record_parser.add_argument("--model", required=True)
    record_parser.add_argument("--show-preview", action="store_true")
    record_parser.add_argument("--output-dir", default="assets")
    record_parser.add_argument("--num-poses", type=int, default=1)

    # Serving subcommand
    serve_parser = subparsers.add_parser("serve", help="Run live pose detection and serve over WebSocket.")
    serve_parser.add_argument("--model", required=True)
    serve_parser.add_argument("--show-preview", action="store_true")
    serve_parser.add_argument("--input-device", type=int, default=0)
    serve_parser.add_argument("--width", type=int, default=1280)
    serve_parser.add_argument("--height", type=int, default=960)
    serve_parser.add_argument("--flip",  action="store_true")
    serve_parser.add_argument("--ip", default="0.0.0.0")
    serve_parser.add_argument("--port", type=int, default=9000)

    args = parser.parse_args()

    if args.command == "record":
        run_record(args)
    elif args.command == "serve":
        run_serve(args)

if __name__ == "__main__":
    main()

