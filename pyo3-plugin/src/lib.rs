use pyo3::prelude::*;
use std::sync::Once;

// Import WASI's malloc and free functions
extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

// Global flag to track if Python runtime is initialized
static INIT: Once = Once::new();

// Global VFS instance - not needed for the export function
// static mut VFS: Option<wasi_vfs::WasiVfs> = None;

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
            // println!("Python runtime initialized and pyo3_plugin module registered");
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
    // println!("Executing Python code: {:?}", code);
    
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

// Import the invoke function from the WASM runtime
#[link(wasm_import_module = "chicory")]
extern "C" {
    fn wasm_invoke(
        module_str_ptr: *const u8,
        module_str_len: usize,
        name_str_ptr: *const u8,
        name_str_len: usize,
        args_str_ptr: *const u8,
        args_str_len: usize,
    ) -> *const u32;
}

// Helper function to call the WASM invoke function
fn invoke_exec(module_str: String, name_str: String, args_str: String) -> String {
    let module_bytes: &[u8] = module_str.as_bytes();
    let name_bytes: &[u8] = name_str.as_bytes();
    let args_bytes: &[u8] = args_str.as_bytes();

    let return_str = unsafe {
        let wide_ptr = wasm_invoke(
            module_bytes.as_ptr(),
            module_bytes.len(),
            name_bytes.as_ptr(),
            name_bytes.len(),
            args_bytes.as_ptr(),
            args_bytes.len(),
        );
        let [ptr, len] = std::slice::from_raw_parts(wide_ptr, 2) else {
            unreachable!()
        };
        let res = std::slice::from_raw_parts(*ptr as *const u8, *len as usize);
        let str_result = std::str::from_utf8(res).unwrap().to_string();
        // Free the memory allocated by the WASM runtime
        plugin_free(*wide_ptr as *mut u8);
        plugin_free(*ptr as *mut u8);
        str_result
    };

    return_str
}

#[pyfunction]
fn invoke(module: String, name: String, args: String) -> PyResult<String> {
    Ok(invoke_exec(module, name, args))
}

#[pymodule]
fn pyo3_plugin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(invoke, m)?)?;
    Ok(())
}

#[export_name = "wizer.initialize"]
pub extern "C" fn init() {
    // Set environment variables to configure Python before initialization
    std::env::set_var("PYTHONHOME", "/usr/local");
    std::env::set_var("PYTHONPATH", "/usr/local/lib/python3.11");
    
    // Initialize Python runtime
    Python::initialize();
    
    // Initialize Python runtime and register our module
    plugin_init();
    
    // Pre-initialize the environment with some basic Python code
    Python::attach(|py| -> PyResult<()> {
        // Set up basic Python environment and preload critical modules
        let init_code = r#"
import sys
print("Python environment pre-initialized")
print(f"Python version: {sys.version}")
print("pyo3_plugin module available for import")
"#;
        
        let c_string = std::ffi::CString::new(init_code).unwrap();
        py.run(&c_string, None, None)
    })
    .expect("Failed to pre-initialize Python environment");
}
