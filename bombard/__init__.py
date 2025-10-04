from functools import wraps
from typing import Callable
from bombard.bombard import select_ok

def bombard(num: int = 1):
    def decorator(func: Callable) -> Callable:
        @wraps(func)
        async def wrapper(*args, **kwargs):
            coroutines = [func(*args, **kwargs) for _ in range(num)]
            return await select_ok(coroutines)

        return wrapper

    return decorator


__all__ = [
    "bombard",
    "select_ok"
]
