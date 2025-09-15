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

// Import specific types from pyo3_ffi to avoid ambiguity with pyo3::prelude
use pyo3_ffi::{
    PyObject, PyObject_Str, PyObject_Repr, PyObject_GetAttr, PyObject_SetAttr,
    Py_IncRef, Py_DecRef, PyUnicode_FromStringAndSize, PyUnicode_AsUTF8AndSize,
    PyUnicode_AsEncodedString, PyUnicode_InternInPlace, PyErr_Restore, PyErr_WriteUnraisable,
    PyErr_SetString, PyErr_PrintEx, PyErr_Print, PyErr_Fetch, PyErr_NormalizeException,
    PyErr_SetObject, PyType_GetName, PyType_GetFlags, PyType_GetQualName, PyBytes_AsString,
    PyBytes_Size, PyList_New, PyList_Append, PyTuple_Size, PyTuple_GetItem,
    PyException_GetTraceback, PyException_SetTraceback, PyException_GetCause,
    PyException_SetCause, PyErr_NewExceptionWithDoc, PyErr_GivenExceptionMatches,
    PyGILState_Release, PyGILState_Ensure, PyEval_SaveThread, PyEval_RestoreThread,
    PyImport_Import, PyObject_CallNoArgs, PyTraceBack_Print, PyNumber_Index,
    PyLong_AsUnsignedLongLong, Py_IsInitialized, PyModule_GetNameObject, PyModule_Create2,
    PyCMethod_New, PyInterpreterState_Get, PyInterpreterState_GetID,
    PyImport_AddModule, Py_XNewRef, PyEval_GetBuiltins, PyDict_SetItem,
    Py_CompileString, PyEval_EvalCode, PySequence_Contains, Py_InitializeEx, PyTuple_New, PyTuple_SetItem,
    PyTypeObject, PyThreadState, PyInterpreterState, PyModuleDef, PyMethodDef,
    Py_ssize_t
};
use std::ffi::{c_char, c_int};

#[no_mangle]
pub extern "C" fn plugin_PyObject_Str(o: *mut PyObject) -> *mut PyObject {
    unsafe { PyObject_Str(o) }
}

#[no_mangle]
pub extern "C" fn plugin__Py_IncRef(o: *mut PyObject) {
    unsafe { Py_IncRef(o) }
}

#[no_mangle]
pub extern "C" fn plugin_PyUnicode_FromStringAndSize(s: *const u8, size: usize) -> *mut PyObject {
    unsafe { PyUnicode_FromStringAndSize(s as *const c_char, size as Py_ssize_t) }
}

#[no_mangle]
pub extern "C" fn plugin__Py_DecRef(o: *mut PyObject) {
    unsafe { Py_DecRef(o) }
}

#[no_mangle]
pub extern "C" fn plugin_PyObject_Repr(o: *mut PyObject) -> *mut PyObject {
    unsafe { PyObject_Repr(o) }
}

#[no_mangle]
pub extern "C" fn plugin_PyErr_Restore(type_: *mut PyObject, value: *mut PyObject, traceback: *mut PyObject) {
    unsafe { PyErr_Restore(type_, value, traceback) }
}

#[no_mangle]
pub extern "C" fn plugin_PyErr_WriteUnraisable(obj: *mut PyObject) {
    unsafe { PyErr_WriteUnraisable(obj) }
}

#[no_mangle]
pub extern "C" fn plugin_PyType_GetName(type_: *mut PyObject) -> *mut PyObject {
    unsafe { PyType_GetName(type_ as *mut PyTypeObject) }
}

#[no_mangle]
pub extern "C" fn plugin_PyObject_GetAttr(obj: *mut PyObject, name: *mut PyObject) -> *mut PyObject {
    unsafe { PyObject_GetAttr(obj, name) }
}

#[no_mangle]
pub extern "C" fn plugin_PyBytes_AsString(obj: *mut PyObject) -> *const u8 {
    unsafe { PyBytes_AsString(obj) as *const u8 }
}

#[no_mangle]
pub extern "C" fn plugin_PyBytes_Size(obj: *mut PyObject) -> usize {
    unsafe { PyBytes_Size(obj) as usize }
}

#[no_mangle]
pub extern "C" fn plugin_PyType_GetFlags(type_: *mut PyObject) -> u32 {
    unsafe { PyType_GetFlags(type_ as *mut PyTypeObject) }
}

#[no_mangle]
pub extern "C" fn plugin_PyErr_GivenExceptionMatches(exc: *mut PyObject, type_: *mut PyObject) -> i32 {
    unsafe { PyErr_GivenExceptionMatches(exc, type_) }
}

#[no_mangle]
pub extern "C" fn plugin_PyList_New(size: usize) -> *mut PyObject {
    unsafe { PyList_New(size as Py_ssize_t) }
}

#[no_mangle]
pub extern "C" fn plugin_PyObject_SetAttr(obj: *mut PyObject, name: *mut PyObject, value: *mut PyObject) -> i32 {
    unsafe { PyObject_SetAttr(obj, name, value) }
}

#[no_mangle]
pub extern "C" fn plugin_PyModule_GetNameObject(module: *mut PyObject) -> *mut PyObject {
    unsafe { PyModule_GetNameObject(module) }
}

#[no_mangle]
pub extern "C" fn plugin_PyUnicode_AsUTF8AndSize(unicode: *mut PyObject, size: *mut usize) -> *const u8 {
    unsafe { PyUnicode_AsUTF8AndSize(unicode, size as *mut Py_ssize_t) as *const u8 }
}

#[no_mangle]
pub extern "C" fn plugin_PyUnicode_AsEncodedString(unicode: *mut PyObject, encoding: *const u8, errors: *const u8) -> *mut PyObject {
    unsafe { PyUnicode_AsEncodedString(unicode, encoding as *const c_char, errors as *const c_char) }
}

#[no_mangle]
pub extern "C" fn plugin_PyImport_Import(name: *mut PyObject) -> *mut PyObject {
    unsafe { PyImport_Import(name) }
}

#[no_mangle]
pub extern "C" fn plugin_PyObject_CallNoArgs(callable: *mut PyObject) -> *mut PyObject {
    unsafe { PyObject_CallNoArgs(callable) }
}

#[no_mangle]
pub extern "C" fn plugin_PyTraceBack_Print(tb: *mut PyObject, f: *mut PyObject) -> i32 {
    unsafe { PyTraceBack_Print(tb, f) }
}

#[no_mangle]
pub extern "C" fn plugin_PyTuple_Size(p: *mut PyObject) -> usize {
    unsafe { PyTuple_Size(p) as usize }
}

#[no_mangle]
pub extern "C" fn plugin_PyTuple_GetItem(p: *mut PyObject, pos: usize) -> *mut PyObject {
    unsafe { PyTuple_GetItem(p, pos as Py_ssize_t) }
}

#[no_mangle]
pub extern "C" fn plugin_PyType_GetQualName(type_: *mut PyObject) -> *mut PyObject {
    unsafe { PyType_GetQualName(type_ as *mut PyTypeObject) }
}

#[no_mangle]
pub extern "C" fn plugin_PyErr_SetString(type_: *mut PyObject, message: *const u8) {
    unsafe { PyErr_SetString(type_, message as *const c_char) }
}

#[no_mangle]
pub extern "C" fn plugin_PyException_GetTraceback(exc: *mut PyObject) -> *mut PyObject {
    unsafe { PyException_GetTraceback(exc) }
}

#[no_mangle]
pub extern "C" fn plugin_PyException_SetTraceback(exc: *mut PyObject, tb: *mut PyObject) -> i32 {
    unsafe { PyException_SetTraceback(exc, tb) }
}

#[no_mangle]
pub extern "C" fn plugin_PyErr_PrintEx(set_sys_last_vars: i32) {
    unsafe { PyErr_PrintEx(set_sys_last_vars) }
}

#[no_mangle]
pub extern "C" fn plugin_PyErr_NewExceptionWithDoc(name: *const u8, base: *mut PyObject, dict: *mut PyObject, doc: *const u8) -> *mut PyObject {
    unsafe { PyErr_NewExceptionWithDoc(name as *const c_char, doc as *const c_char, base, dict) }
}

#[no_mangle]
pub extern "C" fn plugin_PyException_GetCause(exc: *mut PyObject) -> *mut PyObject {
    unsafe { PyException_GetCause(exc) }
}

#[no_mangle]
pub extern "C" fn plugin_PyException_SetCause(exc: *mut PyObject, cause: *mut PyObject) {
    unsafe { PyException_SetCause(exc, cause) }
}

#[no_mangle]
pub extern "C" fn plugin_PyGILState_Release(state: u32) {
    unsafe { PyGILState_Release(std::mem::transmute(state)) }
}

#[no_mangle]
pub extern "C" fn plugin_PyErr_Print() {
    unsafe { PyErr_Print() }
}

#[no_mangle]
pub extern "C" fn plugin_PyEval_SaveThread() -> *mut PyObject {
    unsafe { PyEval_SaveThread() as *mut PyObject }
}

#[no_mangle]
pub extern "C" fn plugin_PyNumber_Index(obj: *mut PyObject) -> *mut PyObject {
    unsafe { PyNumber_Index(obj) }
}

#[no_mangle]
pub extern "C" fn plugin_PyLong_AsUnsignedLongLong(obj: *mut PyObject) -> u64 {
    unsafe { PyLong_AsUnsignedLongLong(obj) }
}

#[no_mangle]
pub extern "C" fn plugin_Py_IsInitialized() -> i32 {
    unsafe { Py_IsInitialized() }
}

#[no_mangle]
pub extern "C" fn plugin_PyGILState_Ensure() -> u32 {
    unsafe { PyGILState_Ensure() as u32 }
}

#[no_mangle]
pub extern "C" fn plugin_PyEval_RestoreThread(tstate: *mut PyObject) {
    unsafe { PyEval_RestoreThread(tstate as *mut PyThreadState) }
}

#[no_mangle]
pub extern "C" fn plugin_PyUnicode_InternInPlace(string: *mut PyObject) {
    unsafe { PyUnicode_InternInPlace(&mut (string as *mut PyObject)) }
}

#[no_mangle]
pub extern "C" fn plugin_PyErr_Fetch(type_: *mut *mut PyObject, value: *mut *mut PyObject, traceback: *mut *mut PyObject) {
    unsafe { PyErr_Fetch(type_, value, traceback) }
}

#[no_mangle]
pub extern "C" fn plugin_PyErr_NormalizeException(type_: *mut *mut PyObject, value: *mut *mut PyObject, traceback: *mut *mut PyObject) {
    unsafe { PyErr_NormalizeException(type_, value, traceback) }
}

#[no_mangle]
pub extern "C" fn plugin_PyErr_SetObject(type_: *mut PyObject, value: *mut PyObject) {
    unsafe { PyErr_SetObject(type_, value) }
}

#[no_mangle]
pub extern "C" fn plugin_PyList_Append(list: *mut PyObject, item: *mut PyObject) -> i32 {
    unsafe { PyList_Append(list, item) }
}

#[no_mangle]
pub extern "C" fn plugin_PyInterpreterState_Get() -> *mut PyObject {
    unsafe { PyInterpreterState_Get() as *mut PyObject }
}

#[no_mangle]
pub extern "C" fn plugin_PyInterpreterState_GetID(interp: *mut PyObject) -> u64 {
    unsafe { PyInterpreterState_GetID(interp as *mut PyInterpreterState) as u64 }
}

#[no_mangle]
pub extern "C" fn plugin_PyModule_Create2(def: *mut PyObject, api_version: i32) -> *mut PyObject {
    unsafe { PyModule_Create2(def as *mut PyModuleDef, api_version as c_int) }
}

#[no_mangle]
pub extern "C" fn plugin_PyCMethod_New(method: *mut PyObject, self_: *mut PyObject, module: *mut PyObject, qualname: *mut PyObject) -> *mut PyObject {
    unsafe { PyCMethod_New(method as *mut PyMethodDef, self_, module, qualname as *mut PyTypeObject) }
}

#[no_mangle]
pub extern "C" fn plugin_PyImport_AddModule(name: *const u8) -> *mut PyObject {
    unsafe { PyImport_AddModule(name as *const c_char) }
}

#[no_mangle]
pub extern "C" fn plugin_Py_XNewRef(obj: *mut PyObject) -> *mut PyObject {
    unsafe { Py_XNewRef(obj) }
}

#[no_mangle]
pub extern "C" fn plugin_PyEval_GetBuiltins() -> *mut PyObject {
    unsafe { PyEval_GetBuiltins() }
}

#[no_mangle]
pub extern "C" fn plugin_PyDict_SetItem(p: *mut PyObject, key: *mut PyObject, val: *mut PyObject) -> i32 {
    unsafe { PyDict_SetItem(p, key, val) }
}

#[no_mangle]
pub extern "C" fn plugin_Py_CompileString(str: *const u8, filename: *const u8, start: i32) -> *mut PyObject {
    unsafe { Py_CompileString(str as *const c_char, filename as *const c_char, start) }
}

#[no_mangle]
pub extern "C" fn plugin_PyEval_EvalCode(co: *mut PyObject, globals: *mut PyObject, locals: *mut PyObject) -> *mut PyObject {
    unsafe { PyEval_EvalCode(co, globals, locals) }
}

#[no_mangle]
pub extern "C" fn plugin_PySequence_Contains(seq: *mut PyObject, item: *mut PyObject) -> i32 {
    unsafe { PySequence_Contains(seq, item) }
}

#[no_mangle]
pub extern "C" fn plugin_Py_InitializeEx(initsigs: i32) {
    unsafe { Py_InitializeEx(initsigs) }
}

#[no_mangle]
pub extern "C" fn plugin_PyTuple_New(size: usize) -> *mut PyObject {
    unsafe { PyTuple_New(size as Py_ssize_t) }
}

#[no_mangle]
pub extern "C" fn plugin_PyTuple_SetItem(p: *mut PyObject, pos: usize, o: *mut PyObject) -> i32 {
    unsafe { PyTuple_SetItem(p, pos as Py_ssize_t, o) }
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
