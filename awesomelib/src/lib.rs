#[macro_use] extern crate cpython;
use cpython::{Python, PyResult};

// logic implemented as a normal rust function
// could be defined in another module
fn sum_as_string(a:i64, b:i64) -> String {
    format!("{}", a + b).to_string()
}

// Note that the py_fn!() macro automatically converts the arguments from
// Python objects to Rust values; and the Rust return value back into a Python object.
fn sum_as_string_py(_: Python, a:i64, b:i64) -> PyResult<String> {
    let out = sum_as_string(a, b);
    Ok(out)
}

py_module_initializer!(awesomelib, initawesomelib, PyInit_awesomelib, |py, m | {
    try!(m.add(py, "__doc__", "This module is implemented in Rust"));
    try!(m.add(py, "sum_as_string", py_fn!(py, sum_as_string_py(a: i64, b:i64))));
    Ok(())
});
