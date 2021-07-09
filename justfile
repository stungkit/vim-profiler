default: fmt ci profile

ci: build test clippy fmt-check

build:
	cargo build

test:
	cargo test

clippy:
  cargo clippy --all-targets --all-features

fmt-check:
  cargo +nightly fmt --all -- --check
  @echo formatting check done

run *args:
	cargo run -- --{{args}}

fmt:
	cargo +nightly fmt

check:
 cargo check

watch +COMMAND='test':
	cargo watch --clear --exec "{{COMMAND}}"

usage:
	cargo run -- --help | pbcopy

profile *args:
	cargo run -- -c neovim {{args}}

plot:
	cargo run -- -c neovim --plot

install:
	cargo install --path .