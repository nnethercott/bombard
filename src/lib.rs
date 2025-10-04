use futures::future;
use pyo3::{prelude::*, types::PyTuple};
use pyo3_async_runtimes::tokio::future_into_py;

use pyo3_stub_gen::{define_stub_info_gatherer, derive::gen_stub_pyfunction};

// an example
#[pyfunction]
fn _select<'py>(coro1: Bound<'py, PyAny>, coro2: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
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

#[gen_stub_pyfunction]
#[pyfunction]
#[pyo3(signature=(*coroutines))]
/// Polls passed coroutines until either one of them returns successfully, or all of them have failed.
///
/// Example:
/// ```python
/// import asyncio
/// import bombardx
///
/// async def ok():
///     await asyncio.sleep(1)
///     return
///
/// async def fail():
///     raise RuntimeError
///
/// _ = bombardx.select_ok(ok(), fail())
/// ```
fn select_ok<'a>(py: Python<'a>, coroutines: &Bound<'a, PyTuple>) -> PyResult<Bound<'a, PyAny>> {
    // Need to collect, otherwise async move can't take ownership of an iter which may be awaken
    // on different threads as `c` is !Send, and pyfunctions don't tolerate trait bounds
    let futures: Vec<_> = coroutines
        .into_iter()
        .map(|c| {
            let fut = pyo3_async_runtimes::tokio::into_future(c).unwrap();
            Box::pin(fut)
        })
        .collect();

    future_into_py(py, async move {
        let (res, _) = future::select_ok(futures).await?;
        return Ok(res);
    })
}

// #[pyfunction]
// fn bombard<'a>(py: Python<'a>, num: usize) -> PyResult<Bound<'a, PyCFunction>> {
//     let decorator = |args: &Bound<'_, PyTuple>,
//                      _kwargs: Option<&Bound<'_, PyDict>>|
//      -> PyResult<Bound<'a, PyCFunction>> {
//         let unbound_func: Py<PyCFunction> = args.get_item(0)?.extract()?;
//         let wrapper = move |args: &Bound<'_, PyTuple>,
//                             kwargs: Option<&Bound<'_, PyDict>>|
//               -> PyResult<Bound<'_, PyAny>> {
//             // Use `Python::attach` to get a `Python` object
//             Python::attach(|py| {
//                 let func = unbound_func.bind(py);
//                 let coroutines: Vec<Bound<'_, PyAny>> =
//                     std::iter::repeat_with(|| func.call(args, kwargs).unwrap())
//                         .take(num)
//                         .collect();
//                 let selected = select_ok(py, coroutines)?;
//                 Ok(selected)
//             })
//         };
//         PyCFunction::new_closure(py, None, None, wrapper)
//     };
//     PyCFunction::new_closure(py, None, None, decorator)
// }

#[pymodule]
#[pyo3(name = "bombardx")]
/// Python bindings for rust's `futures::future::select_ok` function.
fn my_async_module(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(select_ok, m)?)?;
    Ok(())
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
