.PHONY: all build fmt init lint pre-commit test full-test deps

NIGHTLY_TOOLCHAIN_VERSION ?= 2023-03-14
TARGET = `rustc -Vv | grep 'host: ' | sed 's/^host: \(.*\)/\1/'`

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo b -r --workspace
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	@echo ⚙️ Checking a format...
	@cargo fmt --all --check

init:
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup toolchain install nightly-$(NIGHTLY_TOOLCHAIN_VERSION) --component llvm-tools-preview --component clippy
	@rustup target add wasm32-unknown-unknown --toolchain nightly-$(NIGHTLY_TOOLCHAIN_VERSION)
	@rm -rf ~/.rustup/toolchains/nightly-$(TARGET)
	@ln -s ~/.rustup/toolchains/nightly-$(NIGHTLY_TOOLCHAIN_VERSION)-$(TARGET) ~/.rustup/toolchains/nightly-$(TARGET)

lint:
	@echo ⚙️ Running the linter...
	@cargo clippy --workspace --all-targets -- -D warnings

pre-commit: fmt lint full-test

test:
	@echo ⚙️ Running unit tests...
	@cargo t

full-test:
	@echo ⚙️ Running all tests...
	@cargo t -- --include-ignored
