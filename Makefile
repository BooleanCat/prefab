.PHONY: test test-unit test-integration

SHELL := /bin/bash

.DEFAULT_GOAL=build

build:
	cargo build --release

test-unit:
	cargo test

test-integration:
	busted

test: test-unit build test-integration

