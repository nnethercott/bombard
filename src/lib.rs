use futures::future;
use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::future_into_py;

#[pyfunction]
fn rust_select<'py>(
    coro1: Bound<'py, PyAny>,
    coro2: Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let py = coro1.py();
    let fut1 = pyo3_async_runtimes::tokio::into_future(coro1)?;
    let fut2 = pyo3_async_runtimes::tokio::into_future(coro2)?;

    // Wrap Rust future back into a Python awaitable
    future_into_py(py, async move {
        tokio::select! {
            res1 = fut1 => Ok::<_, PyErr>(res1?),
            res2 = fut2 => Ok::<_, PyErr>(res2?),
        }
    })
}

#[pyfunction]
fn rust_select_ok<'a>(py: Python<'a>, coros: Vec<Bound<'a, PyAny>>) -> PyResult<Bound<'a, PyAny>> {
    // need to collect, otherwise async move can't take ownership of an iter which
    // may be awaken on different threads each iter -> `c` is !Send and pyfunctions can't be
    // generic...
    let futs: Vec<_> = coros
        .into_iter()
        .map(|c| {
            let fut = pyo3_async_runtimes::tokio::into_future(c).unwrap();
            Box::pin(fut)
        })
        .collect();

    future_into_py(py, async move {
        let (res, _) = future::select_ok(futs).await?;
        return Ok(res);
    })
}

#[pymodule]
#[pyo3(name = "bombard")]
fn my_async_module(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_select, m)?)?;
    m.add_function(wrap_pyfunction!(rust_select_ok, m)?)?;
    Ok(())
}
