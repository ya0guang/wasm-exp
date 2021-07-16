# Questions

## Memory Size

- Why `0x20000` for `wasmtime/examples/memory.rs`? `assert_eq!(memory.data_size(&store), 0x20000);`

## Globals

### How to generate `global` in Rust?

In [`wasmtime`'s document](https://docs.wasmtime.dev/wasm-rust.html#exporting-rust-functionality)

> Rust currently does not have support for exporting or importing custom global values.

#### Related Threads

- [Rust Github Issue](https://github.com/rust-lang/rust/issues/65987)
- [Stackoverflow question](https://stackoverflow.com/questions/51216284/how-to-store-global-state-in-rust-when-using-wasm-bindgen)

## Safe Buffer Passing in `wasmtime`

See [wasmtime `Memory`](https://docs.wasmtime.dev/api/wasmtime/struct.Memory.html)

To sum up, only use safe APIs(`read` and `write`) and avoid to use pointers after the pages has grown, which may invalidate the previously acquired pointers.