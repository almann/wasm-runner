# Simple WASM Runner

[![Crate](https://img.shields.io/crates/v/wasm-runner.svg)](https://crates.io/crates/wasm-runner)
[![License](https://img.shields.io/hexpm/l/plug.svg)](https://github.com/almann/wasm-runner/blob/main/LICENSE)

This is a simple wrapper program to run a WASM runtime (currently one of [`wasmer`] or [`wasmtime`]) as a runner for
Cargo.  This is useful when wanting to use a target such as `wasm32-wasi` without the trappings of [`wasm-pack`]
for example, just running `cargo run` or `cargo test` in a package for the `wasm32-wasi` target.  Cargo expects a
runner to take the program executable followed by its arguments, and most WASM runtimes have a particular
structure to their arguments (e.g., requiring a `--` before program arguments).

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

## TODO

* Some integration tests/continuous integration with WASM targeted sample code.
* Ability to customize runtime arguments via environment variables.
* Ability to support other runtimes such as [`wavm`].

## License

This project is licensed under the Apache-2.0 License.

[wasmer]: https://wasmer.io/
[wasmtime]: https://wasmtime.dev/
[wasm-pack]: https://github.com/rustwasm/wasm-pack
[wavm]: https://wavm.github.io/