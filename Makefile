SHELL := /bin/bash

run:
	cargo run 15

test: fmt lint
	cargo test

lint:
	cargo clippy

build: fmt lint check
	cargo build

update:
	rustup update
	cargo update

fmt:
	cargo fmt

check:
	cargo check

rustup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
