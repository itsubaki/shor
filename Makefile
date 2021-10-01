SHELL := /bin/bash

run: fmt
	cargo run 15 -1

test: fmt
	cargo test

build: fmt check
	cargo build

update:
	cargo update

fmt:
	cargo fmt

check:
	cargo check

install:
	cargo install --path .

rustup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
