.PHONY: run
run: main
	./main

main: main.c lib.h target/debug/libucontext_bug.a
	cc main.c target/debug/libucontext_bug.a -o main

.PHONY: target/debug/libucontext_bug.a
target/debug/libucontext_bug.a:
	cargo build
