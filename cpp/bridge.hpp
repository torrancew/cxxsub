// A simple "bridge" module that adapts the C++ codebase to our Rust needs

// Include our "library"
#pragma once
#include "lib.hpp"

// Shim method to prove we can invoke Rust-based Notifiers from C++ code
void do_notify(const AbstractNotifier &n) {
  return n.notify();
}

// Forward declaration since this type is also referenced in the generated Rust adapter
class TraitBasedNotifier;

// Include cxx-generated adapters
#pragma once
#include "cxxsub/src/ffi.rs.h"
#include "rust/cxx.h"

// A custom subclass that invokes our Rust-based wrapper type
class TraitBasedNotifier : AbstractNotifier {
  public:
    TraitBasedNotifier(rust::Box<FFINotifier> n) : AbstractNotifier(), n(std::move(n)) {}
    void notify(void) const override { this->n->notify(); }
  private:
    rust::Box<FFINotifier> n;
};

std::unique_ptr<SimpleNotifier> simple_notifier_new() {
  return std::make_unique<SimpleNotifier>();
}

std::unique_ptr<TraitBasedNotifier> trait_based_notifier_new(rust::Box<FFINotifier> n) {
  return std::make_unique<TraitBasedNotifier>(std::move(n));
}
