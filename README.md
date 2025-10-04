# bombard
Python bindings for rust's [select_ok](https://docs.rs/futures/latest/futures/future/fn.select_ok.html) macro.

## Motivation
Async Python currently lacks primitives for polling coroutines until the _first success_. Bombard is useful in situations where users spawn concurrent tasks which may err, but only care that at least one of them completes.

Coroutines heavily dependent on network-induced latencies (e.g. gen-ai applications which aim for fault tolerance through concurrent http retries) can benefit from `bombard.select_ok()` to reduce wait times.

[`asyncio.wait`](https://docs.python.org/3/library/asyncio-task.html#waiting-primitives) gets close, but is unable to distinguish between an exception and a successful result as it simply returns the result of the first future completes:

```python
# some bad code
import asyncio

async def ok(i: int):
  await asyncio.sleep(i)
  print("ok")

async def err(i: int):
  await asyncio.sleep(i)
  raise RuntimeError("oops")

async def main():
    try:
        tasks = [fail(1), ok(2)]
        coroutines = map(asyncio.Task, tasks) # <- fail(1) completes before ok(2) !
        done, pending = await asyncio.wait(coroutines, return_when=asyncio.FIRST_COMPLETED)
    except RuntimeError as e:
        raise e

asyncio.run(main()) # raises a RuntimeError !
```

You could roll your own `select_ok` by repeatedly awaiting `asyncio.wait` and updating `tasks` with `pending` until you've exhausted the list, but this is not ideal.

Contrast the code above with :

```python
import asyncio
import bombard # import Bombard

async def main():
    try:
        done = await bombard.select_ok(fail(1), ok(2)) # <- select first successful
    except RuntimeError as e:
        raise e

asyncio.run(main()) # ignores runtime error and prints "ok" after 2 seconds
```

Bombard also abstracts `select_ok` through a `@bombard(num = ...)` decorator you can add to a fallible async function specifying the desired concurrency level :

```python
import random
from bombard import bombard

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
```
