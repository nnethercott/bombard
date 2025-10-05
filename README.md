# bombard
Python bindings for rust's [select_ok](https://docs.rs/futures/latest/futures/future/fn.select_ok.html) macro.

## Install 
`bombardx` can be installed direcly from PyPI with :
```
uv pip install bombardx
```

## Motivation
Async Python currently lacks primitives for polling coroutines until the _first success_. Bombard is useful in situations where users spawn concurrent tasks which may fail, but only care that at least one of them completes.

Futures heavily dependent on network-induced latencies (e.g. gen-ai applications with concurrent http retries) can benefit from `bombardx.select_ok()` to reduce wait times.

[`asyncio.wait`](https://docs.python.org/3/library/asyncio-task.html#waiting-primitives) gets close, but is unable to distinguish between an exception and a successful result as it simply returns the result of the first future that completes.

For example:

```python
# some bad code
import asyncio
from asyncio import Task

async def ok(i: int):
  await asyncio.sleep(i)
  print("ok")

async def fail():
  raise RuntimeError("oops")

async def main():
    coroutines = [Task(fail()), Task(ok(1))]  # <- fail() completes before ok(1) !
    done, pending = await asyncio.wait(*coroutines, return_when=asyncio.FIRST_COMPLETED)

asyncio.run(main()) # raises a RuntimeError !
```

You _could_ fix this by repeatedly awaiting `asyncio.wait` and updating `tasks` with `pending` until you've exhausted the list, but this is not ideal.

Contrast the code above with :

```python
import asyncio
from bombardx import select_ok

async def main():
    return await select_ok(fail(), ok(1))

asyncio.run(main()) # ignores runtime error and prints "ok" after 2 seconds
```

Bombard also abstracts `select_ok` through a `@bombard(num = ...)` decorator you can add to a fallible async function specifying the desired concurrency level :

```python
import random
from bombardx import bombard

async def random_sleep():
    """sleeps for a random amount of time and returns the duration"""
    t = random.randint(1, 10)
    await asyncio.sleep(t)
    return t

@bombard(num=10)
async def fallible():
  t = await random_sleep()
  if random.random() < 0.5:
    return t
  raise RuntimeError(t)

# spawns the future 10 times and selects the first successful run
asyncio.run(fallible())
```



