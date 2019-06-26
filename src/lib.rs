use libc::*;

#[no_mangle]
unsafe extern "C" fn rust_f(ptr: *mut ucontext_t) {
    let ctx = std::mem::uninitialized::<ucontext_t>();
    println!("Rust writing struct of size {}", std::mem::size_of_val(&ctx));
    std::ptr::write(ptr, ctx);
}
