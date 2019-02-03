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
	cargo clippy $(GENERAL_ARGS) -- -D warnings \
		-A clippy::type-complexity

lint-rustfmt:
	cargo fmt
	git diff --exit-code

run:
	cargo run

test:
	cargo test $(GENERAL_ARGS)

deploy:
	sudo chown -R 1000:1000 $(PWD)
	docker run --rm -it -v $(PWD):/home/rust/src \
		ekidd/rust-musl-builder:latest \
		cargo build --release $(GENERAL_ARGS)
	sudo chown -R $(USER) $(PWD)
	docker build --no-cache -t unibar .
	docker save unibar -o unibar.tar
