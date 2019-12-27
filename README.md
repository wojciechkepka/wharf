# Wharf ⚓🦀
[![GitHub Actions](https://github.com/wojciechkepka/wharf/workflows/Wharf/badge.svg)](https://github.com/wojciechkepka/wharf/actions)
[![crates.io](https://img.shields.io/crates/v/wharf)](https://crates.io/crates/wharf)
[![Crates.io](https://img.shields.io/crates/l/wharf)](https://github.com/wojciechkepka/wharf/blob/master/LICENSE)
[![Docs](https://img.shields.io/badge/docs-master-brightgreen)](https://docs.rs/wharf)

Fully asynchronous docker api library written in Rust.

## Examples
To run examples: 
- clone the repository
  - `git clone https://github.com/wojciechkepka/wharf`
- [run docker daemon listening on tcp port 2376](https://docs.docker.com/engine/reference/commandline/dockerd/)
- run example by replacing `example_name` with one from below
  - `cargo run --example example_name`

Available examples:
- [create_a_container](https://github.com/wojciechkepka/wharf/blob/master/examples/create_a_container.rs)
- [control_a_container](https://github.com/wojciechkepka/wharf/blob/master/examples/control_a_container.rs)

## License
[MIT](https://github.com/wojciechkepka/wharf/blob/master/LICENSE)
