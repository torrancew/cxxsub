mod ffi;
use ffi::*;

struct RustNotifier;

impl Notify for RustNotifier {
    fn notify(&self) {
        println!("Hello from Rust");
    }
}

// Wrap pointer gymnastics for C++ casting in a helper method
// and return, rather than consume, the underlying pointer
unsafe fn do_notify_ptr<T: cxx::private::UniquePtrTarget>(
    n: cxx::UniquePtr<T>,
) -> cxx::UniquePtr<T> {
    let ptr = n.into_raw();
    do_notify(&*(ptr as *const AbstractNotifier));
    cxx::UniquePtr::from_raw(ptr)
}

fn main() {
    let cpp = SimpleNotifier::new();
    let rust = TraitBasedNotifier::new(&RustNotifier);

    // Invoking notify methods from Rust
    cpp.notify();
    rust.notify();

    // Invoking notify methods from C++
    unsafe {
        do_notify_ptr(cpp);
        do_notify_ptr(rust);
    }
}
