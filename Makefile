all: build

bench:
	cargo bench

build:
	cargo build

test:
	cargo test

update:
	cargo update

clean:
	cargo clean

.PHONY: \
	all \
	bench \
	build \
	clean \
	test \
	update
