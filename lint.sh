#!/usr/bin/env bash

project=${PWD##*/}

cargo clean -p "$project"
#cargo clippy -- -W clippy::pedantic -A clippy::module-name-repetitions

cargo fmt -- --check
cargo clippy
