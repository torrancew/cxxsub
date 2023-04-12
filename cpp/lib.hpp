// This is an example C++ "library" providing an abstract class we'd like to subclass
// as well as one concrete subclass for demonstration purposes

#pragma once
#include <memory>
#include <string>
#include <iostream>

// Simple, abstract base class
class AbstractNotifier {
  public:
    explicit AbstractNotifier() {}
    virtual void notify(void) const = 0;
};

// Simple subclass
class SimpleNotifier : public AbstractNotifier {
  public:
    explicit SimpleNotifier() : AbstractNotifier() {}
    void notify(void) const override { std::cout << "Hello from C++" << std::endl; }
};
