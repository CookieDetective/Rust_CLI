# site-scout

`site-scout` is a small Rust CLI for inspecting websites and practicing core Rust
concepts. It includes ten standalone examples in [`basics/`](basics/) and a
networking CLI split into focused modules under [`src/`](src/).

## Build and test

```sh
cargo build
cargo test
```

Each basics file can be compiled and tested on its own:

```sh
rustc --test basics/01_variables.rs -o /tmp/variables-tests
/tmp/variables-tests
```

## Commands

```sh
cargo run -- inspect https://example.com
cargo run -- dns example.com --json
cargo run -- tls example.com
cargo run -- tls example.com --port 8443 --json
```

- `inspect` fetches an HTTP(S) page and reports the final URL, response status,
  content type, and (when present) the page title. It reads at most 1 MiB of
  the response body.
- `dns` resolves a hostname to its IP addresses.
- `tls` connects to a TLS service and reports the peer certificate's DER size.
  Certificate verification is enabled by the platform TLS connector.

Add `--json` to any subcommand for formatted JSON output. Network commands make
real outbound requests to the host you specify.

## Rust practice files

The ten standalone examples cover variables, control flow, functions,
ownership, structs, enums, pattern matching, collections, error handling, and
iterators. Each file includes a small test module and can be compiled with
`rustc --test`.
