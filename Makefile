.PHONY: build
build:
	@cargo build --release

.PHONY: run
run:
	@cargo run

.PHONY: debug
debug:
	@cargo build

.PHONY: test
test:
	@cargo test
