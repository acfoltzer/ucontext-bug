use libc::*;

#[no_mangle]
unsafe extern "C" fn rust_f(ptr: *mut ucontext_t) {
    let ctx = std::mem::uninitialized::<ucontext_t>();
    std::ptr::write(ptr, ctx);
}
