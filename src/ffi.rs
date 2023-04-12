use cxx::UniquePtr;

// Re-export our FFI bindings for use by the rest of this crate
pub(crate) use imp::*;

// We use this trait to enable arbitrary Rust types to "subclass" the C++ AbstractNotifier
pub trait Notify {
    fn notify(&self);
}

// Trait objects can't be passed across the FFI boundary directly, so we wrap them in a newtype
pub struct FFINotifier<'n>(&'n dyn Notify);

// Allow trivial conversion from references to types implementing Notify
impl<'n, T: Notify> From<&'n T> for FFINotifier<'n> {
    fn from(value: &'n T) -> Self {
        Self(value as &dyn Notify)
    }
}

// Expose the inner notify method in the wrapper type
impl FFINotifier<'_> {
    #[inline(always)]
    pub fn notify(&self) {
        self.0.notify()
    }
}

#[cxx::bridge]
mod imp {
    // Expose our trait wrapper to C++
    extern "Rust" {
        type FFINotifier<'n>;
        fn notify(&self);
    }

    // Expose AbstractNotifier to Rust
    unsafe extern "C++" {
        include!("cxxsub/cpp/lib.hpp");

        type AbstractNotifier;
        fn notify(&self);
    }

    // Expose SimpleNotifier to Rust
    unsafe extern "C++" {
        include!("cxxsub/cpp/lib.hpp");

        type SimpleNotifier;
        fn notify(&self);
    }

    // Expose TraitBasedNotifier to Rust
    unsafe extern "C++" {
        include!("cxxsub/cpp/bridge.hpp");

        type TraitBasedNotifier;
        fn notify(&self);
    }

    // Expose various shim methods to Rust
    unsafe extern "C++" {
        fn do_notify(notifier: &AbstractNotifier);

        fn simple_notifier_new() -> UniquePtr<SimpleNotifier>;

        fn trait_based_notifier_new(
            notifier: Box<FFINotifier<'_>>,
        ) -> UniquePtr<TraitBasedNotifier>;
    }
}

// Wrap SimpleNotifier constructor to be more idiomatic
impl SimpleNotifier {
    pub fn new() -> UniquePtr<Self> {
        simple_notifier_new()
    }
}

// Wrap TraitBasedNotifier constructor to be more idiomatic and ergonomic
impl TraitBasedNotifier {
    pub fn new<'n, T: Notify>(notifier: &'n T) -> UniquePtr<Self> {
        trait_based_notifier_new(FFINotifier::from(notifier).into())
    }
}
