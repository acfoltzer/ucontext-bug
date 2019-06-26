#include <ucontext.h>

#include "lib.h"

void f() {
  ucontext_t context;
  rust_f(&context);
}

int main() {
  f();
}
