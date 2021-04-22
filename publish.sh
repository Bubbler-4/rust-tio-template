#!/bin/bash
set -v

wasm-pack build --target web
rm pkg/.gitignore
