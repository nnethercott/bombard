import asyncio
import pytest
import bombard


@pytest.mark.asyncio
async def test_select_ok_works():
    async def fail():
        await asyncio.sleep(0)
        raise RuntimeError

    async def ok():
        await asyncio.sleep(1)
        return True

    res = await bombard.select_ok([ok(), fail()])
    assert res is True


@pytest.mark.asyncio
async def test_select_ok_fails_properly():
    async def fail_after(i: int):
        await asyncio.sleep(1)
        raise RuntimeError

    with pytest.raises(RuntimeError):
        await bombard.select_ok([fail_after(0), fail_after(1)])


@pytest.mark.asyncio
async def test_decorator_spawns_right_num_tasks():
    atomic_count: int = 0

    async def foo():
        nonlocal atomic_count
        asyncio.sleep(1)
        atomic_count += 1
        return

    bar = bombard.bombard(5)(foo)
    await bar()

    assert atomic_count == 5
