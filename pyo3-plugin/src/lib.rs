use pyo3::prelude::*;
use pyo3::ffi::c_str;

#[no_mangle]
pub extern "C" fn eval() -> *const u8 {
    let result = Python::attach(|py| {
        let result = py
            .eval(c_str!("[i * 10 for i in range(5)]"), None, None)
            .map_err(|e| {
                e.print_and_set_sys_last_vars(py);
                e
            })
            .unwrap();
        let res: Vec<i64> = result.extract().unwrap();
        assert_eq!(res, vec![0, 10, 20, 30, 40]);
        format!("{:?}", res)
    });
    
    // Convert to C string and leak the memory
    let c_string = std::ffi::CString::new(result).unwrap();
    c_string.into_raw() as *const u8
}

// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// #[pymodule]
// fn pyo3_plugin(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     Ok(())
// }
