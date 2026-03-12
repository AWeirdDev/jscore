#[unsafe(no_mangle)]
pub extern "C" fn WTFTimer__create(
    _run_loop: *mut std::ffi::c_void,
    _callback: *mut std::ffi::c_void,
    _ctx: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn WTFTimer__deinit(_timer: *mut std::ffi::c_void) {}

#[unsafe(no_mangle)]
pub extern "C" fn WTFTimer__cancel(_timer: *mut std::ffi::c_void) {}

#[unsafe(no_mangle)]
pub extern "C" fn WTFTimer__update(_timer: *mut std::ffi::c_void, _seconds: f64, _repeat: bool) {}

#[unsafe(no_mangle)]
pub extern "C" fn WTFTimer__isActive(_timer: *mut std::ffi::c_void) -> bool {
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn WTFTimer__secondsUntilTimer(_timer: *mut std::ffi::c_void) -> f64 {
    0.0
}

#[unsafe(no_mangle)]
pub extern "C" fn Bun__errorInstance__finalize(_ptr: *mut std::ffi::c_void) {}

#[unsafe(no_mangle)]
pub extern "C" fn Bun__reportUnhandledError(
    _global: *mut std::ffi::c_void,
    _value: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn Bun__thisThreadHasVM() -> bool {
    false
}
