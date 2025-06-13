import asyncio
import websockets

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
