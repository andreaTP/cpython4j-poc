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
    println!("DEBUG: Starting example() function");
    
    // First, ensure Python runtime is initialized
    println!("DEBUG: Initializing Python runtime");
    Python::initialize();
    println!("DEBUG: Python runtime initialized");


    let _ = Python::attach(|py| -> PyResult<()> {
        let result = py
            .eval(pyo3::ffi::c_str!("[i * 10 for i in range(5)]"), None, None)
            .map_err(|e| {
                e.print_and_set_sys_last_vars(py);
                e
            })?;
        let res: Vec<i64> = result.extract().unwrap();
        assert_eq!(res, vec![0, 10, 20, 30, 40]);
        Ok(())
    });

    println!("DEBUG: example() function completed successfully");
    
    // Python::attach(|py| {
    //     println!("DEBUG: Python::attach() called successfully");
        
    //     let c_string = match std::ffi::CString::new("import pyo3_example\npyo3_example.sum_as_string(5, 20)") {
    //         Ok(s) => {
    //             println!("DEBUG: CString created successfully: {:?}", s);
    //             s
    //         },
    //         Err(e) => {
    //             println!("DEBUG: Failed to create CString: {}", e);
    //             panic!("Failed to create CString: {}", e);
    //         },
    //     };
        
    //     println!("DEBUG: About to execute Python code");
        
    //     // First try to import the module
    //     println!("DEBUG: Importing pyo3_example module");
    //     // let import_cstr = std::ffi::CString::new("import pyo3_example").unwrap();
    //     let import_cstr = std::ffi::CString::new("print(\"debug me\")").unwrap();
    //     let import_result = py.run(&import_cstr, None, None);
    //     match import_result {
    //         Ok(_) => println!("DEBUG: Module import succeeded"),
    //         Err(e) => {
    //             println!("DEBUG: Module import failed: {:?}", e);
    //             e.print_and_set_sys_last_vars(py);
    //             panic!("Module import failed: {:?}", e);
    //         }
    //     }
        
    //     // Then call the function
    //     println!("DEBUG: Calling sum_as_string function");
    //     let eval_cstr = std::ffi::CString::new("pyo3_example.sum_as_string(5, 20)").unwrap();
    //     let result = py.eval(&eval_cstr, None, None);
        
    //     match result {
    //         Ok(_) => {
    //             println!("DEBUG: Python execution succeeded");
    //         },
    //         Err(e) => {
    //             println!("DEBUG: Python execution failed with error: {:?}", e);
    //             println!("DEBUG: Error type: {}", std::any::type_name_of_val(&e));
                
    //             // Try to get more details about the error
    //             let err_type = e.get_type(py);
    //             println!("DEBUG: Error type name: {:?}", err_type.name());
                
    //             let err_value = e.value(py);
    //             println!("DEBUG: Error value: {:?}", err_value);
                
    //             if let Some(err_traceback) = e.traceback(py) {
    //                 println!("DEBUG: Error traceback: {:?}", err_traceback);
    //             } else {
    //                 println!("DEBUG: No traceback available");
    //             }
                
    //             // Print the error to stderr
    //             e.print_and_set_sys_last_vars(py);
                
    //             println!("DEBUG: About to panic with error");
    //             panic!("Python execution failed: {:?}", e);
    //         }
    //     }
    // });
    
    // println!("DEBUG: example() function completed successfully");
}
