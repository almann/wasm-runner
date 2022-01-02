# Simple WASM Runner

[![Crate](https://img.shields.io/crates/v/wasm-runner.svg)](https://crates.io/crates/wasm-runner)
[![CI Build](https://github.com/almann/wasm-runner/workflows/CI%20Build/badge.svg)](https://github.com/almann/wasm-runner/actions?query=workflow%3A%22CI+Build%22)
[![License](https://img.shields.io/hexpm/l/plug.svg)](https://github.com/almann/wasm-runner/blob/main/LICENSE)

This is a simple wrapper program to run a WASM runtime (currently one of [`wasmer`][wasmer] or [`wasmtime`][wasmtime])
as a runner for things like `cargo run`.  This is useful when wanting to use a target such as `wasm32-wasi` without the
trappings of [`wasm-pack`][wasm-pack] for example, just running `cargo run` or `cargo test` in a package
for the `wasm32-wasi` target.  Cargo expects a runner to take the program executable followed by its arguments,
and most WASM runtimes  have a particular structure to their arguments
(e.g., requiring a `--` before program arguments).

Something like [`cargo-wasi`][cargo-wasi] is a great, more integrated solution, but if you want or need control over how the
WASM runtime is executed (e.g., passing runtime options down to the runner), this simple wrapper may meet your needs.

## Usage

Install the runner using cargo:
```
$ cargo install wasm-runner
```

You can install directly from the source tree as:
```
$ cargo install --path=.
```

After this you can now have a `.cargo/config.toml` that uses this runner to execute your binaries for WASM targets.

For example:
```
[target.'cfg(target_arch="wasm32")']
runner = ["wasm-runner", "wasmer"]
```

This assumes `wasmer` is in your `PATH`, but you can also specify the full path if you like.

At this point assuming `wasm32-wasi` is installed as a target in your toolchain (e.g., `rustup target add wasm32-wasi`),
you can run unit tests or binaries with:
```
$ cargo --target wasm32-wasi test
$ cargo --target wasm32-wasi run -- some arguments
```

There are some environment variables that can add additional configurability
(e.g., the `[env]` Cargo configuration key):

* The `WASM_RUNNER_VERBOSE` variable can be set (to any value such as `1`)to get diagnostic output as to what 
  the runner is doing.
* The `WASM_RUNNER_RT_ARGS` environment variable can take a JSON array of strings to pass runtime arguments.

You can run everything without a configuration file if you so choose:  

```
$ WASM_RUNNER_RT_ARGS='["--enable-all", "--llvm"]' \
    WASM_RUNNER_VERBOSE=1 \
    CARGO_TARGET_WASM32_WASI_RUNNER="wasm-runner wasmer" \
    cargo run --target=wasm32-wasi -- hello world
```

## Sample Application

The `test-wasm` folder has a simple crate that is configured to run the wrapper with `wasmer`.
```
$ (cd test-wasm && cargo run --verbose -- hello world)
```

## TODO

* CI testing for `wasmtime`.
* Ability to support other runtimes such as [`wavm`][wavm].

## License

This project is licensed under the Apache-2.0 License.

[cargo-wasi]: https://crates.io/crates/cargo-wasi
[wasmer]: https://wasmer.io/
[wasmtime]: https://wasmtime.dev/
[wasm-pack]: https://github.com/rustwasm/wasm-pack
[wavm]: https://wavm.github.io/
