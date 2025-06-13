import asyncio
import struct

class Server:
    def __init__(self, host='0.0.0.0', port=9000, queue=asyncio.Queue()):
        self.host = host
        self.port = port
        self.queue = queue

    async def handle_client(self, _, writer):
        while True:
            msg = await self.queue.get()
            if msg is None:
                print("Send loop received shutdown signal.")
                break
            writer.write(struct.pack('<I', len(msg)) + msg)
            try:
                await writer.drain()
            except ConnectionResetError:
                print("Client connection reset.")
                break
        writer.close()
        #await writer.wait_closed()

    async def run(self):
        server = await asyncio.start_server(self.handle_client, self.host, self.port)
        async with server:
            print(f"Serving on {server.sockets[0].getsockname()}")
            await server.serve_forever()

