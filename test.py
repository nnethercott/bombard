from bombard import bombard
import asyncio
import random


async def fail(i):
    await asyncio.sleep(i)
    print(i)
    raise RuntimeError("ahhh")


async def ok():
    await asyncio.sleep(2)
    print("ok")


# this is an example of why asyncio.wait is bad!
async def main():
    try:
        coroutines = map(asyncio.Task, [fail(0), ok()])
        await asyncio.wait(coroutines, return_when=asyncio.FIRST_COMPLETED)
    except RuntimeError as e:
        print(e)


async def random_sleep():
    t = random.randint(1, 10)
    await asyncio.sleep(t)
    print(t)


@bombard(num=10)
async def nate():
    await random_sleep()


async def foo():
    await nate()


async def bar():
    await asyncio.gather(*[random_sleep() for _ in range(10)])


print("running select_ok")
asyncio.run(foo())

print("running asyncio.gather")
asyncio.run(bar())
