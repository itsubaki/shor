SHELL := /bin/bash

run:
	# N=15, a=0, t=3
	cargo run 15 0 3

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

install: fmt lint
	cargo install --path .

rustup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
