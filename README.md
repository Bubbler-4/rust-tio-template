# rust-tio-template

A template for a Rust-powered static-page Try Online interface

## What is included

This is an example setup that enables all of the following at once:

* Develop the core interpreter as a Rust library (`lib.rs` and submodules).
* Develop the CLI wrapper as a Rust binary (`main.rs` and submodules).
* Build the core interpreter into Wasm and load from `index.html`.

A proof-of-concept code for parsing with [`nom`](https://docs.rs/nom/6.1.2/nom/) and exporting an opaque JS class is also included.

## Usage

Run `cargo run` to build and run the CLI app.

run `start.sh` to build the Wasm package and start a local web server (so that you can check how the code runs in the browser).
`wasm-pack` produces the Wasm package and the necessary JS glue code out of the box, so you can simply load them from `index.html`.

The default `index.html` has nothing visible, but it contains the minimal JS example that calls Wasm code and `console.log`s the results.

The setup uses GitHub Pages to host the static page. When your code (on the `main` branch) is ready, switch to `gh-pages` branch,
merge from `main`, and run `publish.sh` (which will build the package and expose the build artifacts to Git) and commit.

## Tips

Due to how Wasm is designed, the types that can be passed across Wasm boundary are severely limited
(basically, single primitive integer or floating value, a slice of them, or a string).
In order to keep and pass around more complex data, Wasm-exported struct does the job (which gets exposed as a ), as in the example lib code.