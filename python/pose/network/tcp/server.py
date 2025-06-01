import asyncio
import socket

class Server:
    def __init__(self, host='0.0.0.0', port=9000):
        self._host = host
        self._port = port
        self._server_socket = None
        self._writer = None

    async def start(self):
        # Create listening socket
        self._server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self._server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        self._server_socket.bind((self._host, self._port))
        self._server_socket.listen(1)
        self._server_socket.setblocking(False)

        print(f"Server listening on {self._host}:{self._port} ... Waiting for client.")

        # Accept one client connection
        loop = asyncio.get_running_loop()
        client_socket, addr = await loop.sock_accept(self._server_socket)
        print(f"Client connected from {addr}")

        # Wrap in asyncio streams
        _, writer = await asyncio.open_connection(sock=client_socket)
        self._writer = writer

    async def send_loop(self, queue: asyncio.Queue):
        while True:
            msg = await queue.get()
            if msg is None:
                print("Send loop received shutdown signal.")
                break
            success = await self.send(msg)
            if not success:
                print("Client disconnected. Exiting send loop.")
                break

    async def send(self, msg: bytes) -> bool:
        if self._writer:
            try:
                self._writer.write(msg)
                await self._writer.drain()
                return True
            except (ConnectionResetError, BrokenPipeError):
                print("Client disconnected.")
                self._writer = None
        return False

    async def shutdown(self):
        if self._writer:
            self._writer.close()
            await self._writer.wait_closed()
            self._writer = None
        if self._server_socket:
            self._server_socket.close()
            self._server_socket = None
        print("Server shut down.")
