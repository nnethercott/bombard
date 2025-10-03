use std::ffi::{c_char, CStr};

use futures::future;
use pyo3::{
    prelude::*,
    types::{PyCFunction, PyDict, PyTuple},
};
use pyo3_async_runtimes::tokio::future_into_py;

// an example
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
    // Need to collect, otherwise async move can't take ownership of an iter which may be awaken
    // on different threads as `c` is !Send, and pyfunctions don't tolerate trait bounds
    let futures: Vec<_> = coros
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
// fn bombard(py: Python, num: usize) -> PyResult<Py<PyCFunction>> {
//     // this is our wrapper
//     let wrapper = move |py: Python, func: Py<PyCFunction>| -> PyResult<Py<PyCFunction>> {
//         let f = move |args: &Bound<'_, PyTuple>,
//                       kwargs: Option<&Bound<'_, PyDict>>|
//               -> PyResult<Py<PyAny>> {
//             Python::attach(|py| {
//                 let coroutines: Vec<Bound<'_, PyAny>> =
//                     std::iter::repeat_with(|| func.call(py, args, kwargs).unwrap().into_bound(py))
//                         .take(num)
//                         .collect();
//
//                 let selected = rust_select_ok(py, coroutines)?;
//                 Ok(selected.unbind())
//             })
//         };
//
//         let bound = PyCFunction::new_closure(py, None, None, f)?;
//         Ok(bound.unbind())
//     };
//
//     PyCFunction::new_closure(py, None, None, wrapper)
// }

#[pymodule]
#[pyo3(name = "bombard")]
fn my_async_module(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_select_ok, m)?)?;
    m.add_function(wrap_pyfunction!(bombard, m)?)?;
    Ok(())
}
