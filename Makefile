# Compiler configuration
GENERAL_ARGS = --all

.PHONY: \
	bench \
	build \
	build-doc \
	coverage \
	lint-clippy \
	lint-rustfmt \
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
	cargo clippy $(GENERAL_ARGS) -- \
		-D warnings \
		-A clippy::type-complexity

lint-rustfmt:
	cargo fmt
	git diff --exit-code

run:
	cargo run

test:
	cargo test $(GENERAL_ARGS)
