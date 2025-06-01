import struct

async def forward(source, queue, parser):
    async for item in source:
        parsed = parser(item)
        if parsed is None:
            continue
        framed = struct.pack('<I', len(parsed)) + parsed
        await queue.put(framed)
