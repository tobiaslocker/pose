async def forward(source, queue, parser):
    async for item in source:
        parsed = parser(item)
        if parsed is None:
            continue
        await queue.put(parsed)
