use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_example(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}


#[no_mangle]
pub extern "C" fn example() {
    Python::attach(|py| {
        let c_string = match std::ffi::CString::new("import pyo3_example\npyo3_example.sum_as_string(5, 20)") {
            Ok(s) => s,
            Err(e) => panic!("Failed to create CString: {}", e),
        };
        
        // Use run() for multi-line scripts instead of eval() for single expressions
        let _result = py
            .run(&c_string, None, None)
            .map_err(|e| {
                e.print_and_set_sys_last_vars(py);
                e
            })
            .unwrap_or_else(|e| panic!("Python execution failed: {:?}", e));
    });
}
