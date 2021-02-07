SHELL := /bin/sh


.PHONY: preinstall build-full-blocking build-rustls-tls build-full-async test-full-async test-full-blocking test-rustls-tls

preinstall:
	bash preinstall.sh

build-no-default-features: preinstall
	# Check "no default features"
	cargo build --verbose --workspace --exclude binary_size --no-default-features --features default-tls

build-full-blocking: preinstall
	# Check "full/blocking"
	cargo build --verbose --workspace --exclude binary_size

test-full-blocking: build-full-blocking
	cargo test --verbose --workspace --exclude binary_size

build-full-async: preinstall
	# Check "full/async"
	cargo build --verbose --features async --workspace --exclude binary_size

test-full-async: build-full-async
	cargo test --verbose --features async --example async_create_charge

build-rustls-tls: preinstall
	# Check "full/blocking"
	cargo build --verbose --no-default-features --features "full webhook-events blocking rustls-tls" --workspace --exclude binary_size

test-rustls-tls: build-rustls-tls
	cargo test --verbose --no-default-features --features "full webhook-events blocking rustls-tls" --workspace --exclude binary_size

all: test-full-blocking test-full-async test-rustls-tls