
.PHONY: run
run: out/main
	out/main

out/main: csrc/main.c target/release/libc_unwind_example.a
	mkdir -p out
	gcc -o $@ $^

target/release/libc_unwind_example.a: cargo_build

.PHONY: cargo_build
cargo_build:
	cargo build --release -q