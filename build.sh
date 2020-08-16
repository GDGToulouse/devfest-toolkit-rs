#!/usr/bin/env bash

set -e

RUST_LOG=info

echo "🦀 Check formatting..."
cargo fmt -- --check

echo "🎩 Linting..."
cargo check --all
cargo clippy --all

echo "🧪 Testing..."
cargo test --all

echo "🐳 building docker image..."
docker build -t ilaborie/devfest-toolkit-rs .
