all: build

build:
	cargo build --release

test:
	cargo test

update:
	cargo update

clean:
	cargo clean

.PHONY: \
	all \
	build \
	clean \
	test \
	update
