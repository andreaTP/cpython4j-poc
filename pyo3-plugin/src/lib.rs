use pyo3::prelude::*;
use std::sync::Once;

// Import WASI's malloc and free functions
extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

// Global flag to track if Python runtime is initialized
static INIT: Once = Once::new();

// Re-export WASI's malloc and free functions with different names
#[no_mangle]
pub extern "C" fn plugin_malloc(size: usize) -> *mut u8 {
    unsafe { malloc(size) }
}

#[no_mangle]
pub extern "C" fn plugin_free(ptr: *mut u8) {
    unsafe { free(ptr) }
}

// Initialize Python runtime and register the module
#[no_mangle]
pub extern "C" fn plugin_init() {
    INIT.call_once(|| {
        // Initialize the Python interpreter
        Python::initialize();
        
        Python::attach(|py| {
            // Create and register the pyo3_plugin module
            let module = PyModule::new(py, "pyo3_plugin").unwrap();
            pyo3_plugin(&module).unwrap();
            
            py.import("sys")
                .unwrap()
                .getattr("modules")
                .unwrap()
                .set_item("pyo3_plugin", module)
                .unwrap();
            println!("Python runtime initialized and pyo3_plugin module registered");
        });
    });
}

#[no_mangle]
pub extern "C" fn plugin_eval(ptr: *const u8, length: usize) {
    // Check if runtime is initialized
    if !INIT.is_completed() {
        panic!("Python runtime not initialized. Call plugin_init() first.");
    }
    
    // Read string from the provided memory
    let slice = unsafe { std::slice::from_raw_parts(ptr, length) };
    let code = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(e) => panic!("Invalid UTF-8 string: {}", e),
    };
    
    // Debug: print what we're about to execute
    println!("Executing Python code: {:?}", code);
    
    // Execute the Python code
    Python::attach(|py| {
        let c_string = match std::ffi::CString::new(code) {
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
        // Ignore the result - we don't need to return anything
    });
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn pyo3_plugin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

