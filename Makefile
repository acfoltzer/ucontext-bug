.PHONY: run
run: main
	./main

main: main.c lib.h target/debug/libucontext_bug.so
	cc main.c target/debug/libucontext_bug.so -o main

.PHONY: target/debug/libucontext_bug.so
target/debug/libucontext_bug.so:
	cargo build
