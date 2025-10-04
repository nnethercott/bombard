from functools import wraps
from typing import Callable
from bombardx.bombardx import select_ok


def bombard(num: int = 1):
    """
    A decorator spawning the wrapped function num-times and selecting the first result that is not an exception.

    Example:
    ```python
    from bombardx import bombard
    from pydantic import BaseModel

    class Model(BaseModel):
        pass

    @bombard(num=3)
    async def my_future():
        res = await async_client.get("<some-url>")
        model = Model(**res.json())
        return model
    ```
    """

    def decorator(func: Callable) -> Callable:
        @wraps(func)
        async def wrapper(*args, **kwargs):
            coroutines = (func(*args, **kwargs) for _ in range(num))
            return await select_ok(*coroutines)

        return wrapper

    return decorator


__all__ = ["bombard", "select_ok"]
