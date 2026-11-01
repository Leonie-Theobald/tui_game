#!/bin/bash
set -e

echo "🦀 Installing Rust toolchain..."
rustup update

echo "📦 Fetching dependencies..."
cargo fetch

echo "✅ DevContainer ready!"