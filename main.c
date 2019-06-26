#include <stdio.h>
#include <ucontext.h>

#include "lib.h"

void f() {
  ucontext_t context;
  printf("C passing pointer to a struct of size %ld\n", sizeof(ucontext_t));
  rust_f(&context);
}

int main() {
  f();
}
