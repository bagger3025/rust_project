#!/bin/zsh

cargo build && leaks --atExit -q -- ./target/debug/rust_project
