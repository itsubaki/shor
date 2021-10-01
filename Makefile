SHELL := /bin/bash

run: fmt
	# N=15, a=-1, t=3
	cargo run 15 -1 3

test: fmt
	cargo test

build: fmt check
	cargo build

update:
	rustup update
	cargo update

fmt:
	cargo fmt

check:
	cargo check

install:
	cargo install --path .

rustup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
