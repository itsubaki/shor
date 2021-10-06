SHELL := /bin/bash

run: lint
	# N=15, a=0, t=3
	cargo run 15 0 3

test: lint
	cargo test

lint: fmt
	cargo clippy

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
