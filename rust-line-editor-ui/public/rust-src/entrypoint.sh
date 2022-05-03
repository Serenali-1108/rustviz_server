#! /usr/bin/env bash

echo "$@" > $HOME/src/main.rs
cargo check

