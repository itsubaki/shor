SHELL := /bin/bash

run:
	cargo run 15

build: check
	cargo build

update:
	cargo update

check:
	cargo check

rustup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
