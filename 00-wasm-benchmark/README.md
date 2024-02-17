# WASM Benchmark

This project compares the sequential read and write speeds between the same
code compiled for native and WASM.

## Native

```shell
cargo bench --profile release
```

## WASM

This is more complicated since Criterion.rs outputs files. So, we need the WASM
runtime to open up some directories.

```shell
cargo wasi build --benches --release
wasmtime --dir 'target/criterion/read/new' \
    --dir 'target/criterion/read/base' \
    --dir 'target/criterion/read' \
    --dir 'target/criterion/write/new' \
    --dir 'target/criterion/write/base' \
    --dir 'target/criterion/write' \
    -- \
    ./target/wasm32-wasi/release/deps/write_benchmark-<hash>.wasm \
    --bench
```
