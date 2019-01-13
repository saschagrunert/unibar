# Compiler configuration
GENERAL_ARGS = --all

.PHONY: \
	bench \
	build-doc \
	build \
	coverage \
	lint-rustfmt \
	lint-clippy \
	run \
	test

ifndef VERBOSE
.SILENT:
else
GENERAL_ARGS += -v
endif

all: build

bench:
	cargo bench $(GENERAL_ARGS)

build:
	cargo build $(GENERAL_ARGS)

build-doc:
	cargo doc --no-deps $(GENERAL_ARGS)

coverage:
	cargo kcov

lint-clippy:
	cargo clippy $(GENERAL_ARGS) -- -D warnings

lint-rustfmt:
	cargo fmt
	git diff --exit-code

run:
	cargo run $(GENERAL_ARGS)

test:
	cargo test $(GENERAL_ARGS)
