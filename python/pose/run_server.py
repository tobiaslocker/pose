import asyncio
from pose.network.ws.server import Server
from pose.detection import PoseLandmarkDetection
from pose.network import forward
from pose.protocol.fbs.detection import parse
import argparse
from pose.visualize.mp_drawing import draw_result
import websockets
import os
import json

NUM_PERFORMERS = 1
RECORDING_DIR = "recordings"
os.makedirs(RECORDING_DIR, exist_ok=True)

# In-memory storage
recordings = [[{ "name": f"performer_{i}", "frames": [] }] for i in range(NUM_PERFORMERS)]

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
            print(f"Frame at {result.timestamp}ms")
            if result.success:
                draw_result(result)
                # Collect in memory
                for idx, landmarks in enumerate(result.result.pose_landmarks):
                    if idx < NUM_PERFORMERS:
                        frame_data = capture_frame(result.timestamp, landmarks)
                        recordings[idx].append(frame_data)
            yield result

    if args.show_preview:
        source = with_preview(source)

    if args.input_video:
        async for _ in source:
            pass  # Just inference + preview
        # Write once at the end
        for idx, frames in enumerate(recordings):
            filename = os.path.join(RECORDING_DIR, f"performer_{idx+1}.json")
            with open(filename, "w") as f:
                json.dump(frames, f, indent=2)
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

#async def main():
#    parser = argparse.ArgumentParser()
#    parser.add_argument("--model", required=True)
#    parser.add_argument("--show-preview", action="store_true") 
#    parser.add_argument("--input-video", help="Path to input video file instead of webcam")
#    args = parser.parse_args()
#
#    if args.input_video:
#        inference = PoseLandmarkDetection.fromfilename(args.input_video, args.model)
#    else:
#        inference = PoseLandmarkDetection.fromindex(0, args.model)
#
#    source = inference.to_generator()
#
#    async def with_preview(source):
#        async for result in source:
#            print(f"Frame at {result.timestamp}ms")
#            if result.success:
#                draw_result(result)
#            yield result
#
#    if args.show_preview:
#        source = with_preview(source)
#
#    queue = asyncio.Queue(maxsize=32)
#    server = Server(host="127.0.0.1", port=9000, queue=queue)
#
#    server_runner = await websockets.serve(
#        server.handle_client, server.host, server.port, ping_interval=None
#    )
#
#    forward_task = asyncio.create_task(forward(source, queue, parse))
#    await forward_task
#
#    server_runner.close()
#    await server_runner.wait_closed()
#
#    try:
#        await server.run()
#    except asyncio.CancelledError:
#        pass

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("Server stopped cleanly.")

