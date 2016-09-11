Example from https://github.com/Ogeon/rustful ready to build. 

Install ssl dev https://github.com/sfackler/rust-openssl#building

```bash
cargo build
./target/debug/rustful-example
```

Open http://localhost:8085 in web browser.

## Troubleshooting

### error: could not find native static library `openssl_shim`, perhaps an -L flag is missing?

> May be necessary clean the repository with `cargo clean` before build again.
> -- https://github.com/sfackler/rust-openssl
