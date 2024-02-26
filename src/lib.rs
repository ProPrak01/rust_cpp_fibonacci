#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        fn fibonacci(n: u64) -> u64;
    }
}

pub fn call_fibonacci(n: u64) -> u64 {
    unsafe { ffi::fibonacci(n) }
}