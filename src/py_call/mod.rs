use pyo3:: prelude::*;
use pyo3::types::PyTuple;
use pyo3::types::PyDict;
use pyo3::types::PyString;

pub fn call_with_no_args<N, T>(py_obj: &PyObject, name: N, default: T) -> T
where
    for<'a> N: IntoPyObject<'a, Target = PyString>,
    for<'b> T: FromPyObject<'b>,
{
    call_method_with_kwargs(py_obj, name, (), default, None)
}

pub fn call_method<N, A, T>(py_obj: &PyObject, name: N, args: A, default: T) -> T
where
    for<'a> N: IntoPyObject<'a, Target = PyString>,
    for<'b> A: IntoPyObject<'b, Target = PyTuple, Output = Bound<'b, PyTuple>>,
    for<'c> T: FromPyObject<'c>,
{
    call_method_with_kwargs(py_obj, name, args, default, None)
}

pub fn call_method_with_kwargs<'py, N, A, T>(
    py_obj: &PyObject,
    name: N,
    args: A,
    default: T,
    kwargs: Option<&Bound<'py, PyDict>>) -> T
where
    for<'a> N: IntoPyObject<'a, Target = PyString>,
    for<'b> A: IntoPyObject<'b, Target = PyTuple, Output = Bound<'b, PyTuple>>,
    for<'c> T: FromPyObject<'c>,
{
    Python::with_gil(|py| {
        match py_obj.call_method(py, name, args, kwargs) {
            Ok(v) => v.extract(py).unwrap_or(default),
            Err(_) => default,
        }
    })
}