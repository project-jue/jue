# Makefile for entire Jue workspace

.PHONY: all build clean test run runjit repl

all: build

build:
	cargo build --workspace

release:
	cargo build --workspace --release

test:
	cargo test --workspace

clean:
	cargo clean

# Run compiler (juec)
runc:
	cargo run -p juec -- $(ARGS)

# Run interpreter/runtime (juerun)
run:
	cargo run -p juerun -- $(ARGS)

# Example: Jue REPL
repl:
	cargo run -p juerun -- repl

# Example: Build & run a sample Jue file
runfile:
	cargo run -p juec -- $(FILE)
	cargo run -p juerun -- out.jbc
