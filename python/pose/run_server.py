import asyncio
#from pose.network.tcp.server import Server
from pose.network.ws.server import Server
from pose.detection import PoseLandmarkDetection
from pose.network import forward
from pose.protocol.fbs.detection import parse
import argparse
from pose.visualize.mp_drawing import draw_result

async def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--model", required=True)
    parser.add_argument("--show-preview", action="store_true")
    args = parser.parse_args()

    inference = PoseLandmarkDetection(args.model)
    source = inference.to_generator()

    async def with_preview(source):
        async for result in source:
            success, frame = inference.cap.read()
            if success:
                draw_result(result, frame)
            yield result

    if args.show_preview:
        source = with_preview(source)

    queue = asyncio.Queue(maxsize=32)
    server = Server(host="127.0.0.1", port=9000, queue=queue)

    asyncio.create_task(forward(source, queue, parse))
    try:
        await server.run()
    except asyncio.CancelledError:
        pass

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("Server stopped cleanly.")

