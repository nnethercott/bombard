import bombard
from bombard import bombard
import asyncio


async def fail(i):
    await asyncio.sleep(i)
    print(i)
    raise RuntimeError("ahhh")


async def ok():
    await asyncio.sleep(1)
    print("ok")


async def main():
    # await bombard.rust_select_ok([fail(3), ok()])
    # await asyncio.gather(*[fail(), ok()])
    try:
        coros = map(asyncio.Task, [fail(0), ok()])
        await asyncio.wait(coros, return_when=asyncio.FIRST_COMPLETED)
    except RuntimeError as e:
        print(e)


@bombard(num=10)
async def nate():
    await asyncio.sleep(1)
    print("nate")

async def foo():
    await nate()

asyncio.run(foo())


# OBJECTIVE:
# @bombard(n_concurrent=5)
# async def foo():
#   await pass
