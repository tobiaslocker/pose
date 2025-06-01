import struct

async def forward(source, queue, parser):
    async for item in source:
        parsed = parser(item)
        if parsed is None:
            continue
        framed = struct.pack('<I', len(parsed)) + parsed  # NOTE: Little-endian to match Rust u32::from_le_bytes
        await queue.put(framed)



#async def forward(source, queue, parser):
#    async for item in source:
#        parsed = parser(item)
#        await queue.put(parsed)

