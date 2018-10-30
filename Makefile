.PHONY: test test-unit test-integration test-deps

SHELL := /bin/bash

.DEFAULT_GOAL=build

build:
	cargo build --release

test-unit:
	cargo test

test-integration:
	busted

test: test-unit build test-integration

test-deps:
	while read in; do luarocks install "$$in"; done < .rocks

