# ontime

`ontime` implements a solver for punctual reachability games on temporal graphs using simple backwards induction.

## Quick start

Assuming you have [rust and cargo installed](https://www.rust-lang.org/tools/install), compile and run shepherd as follows.

```
cargo run -- examples/game1.1.tg "s,t" 10000
```

That will load a temporal graph from the file `examples/game1.1.tg`
and compute which states at time 0 can force to visit the target $\{s,t\}$ at time 10000.


## Installation

You can build an optimized binary (will be placed in `target/release/ontime`) using the following command.

```
cargo build --release
```

To install the binary for later use this:
```
cargo install --path .  # to install the binary to your $PATH
```
This will move the binary into to your cargo path, usually `~/.cargo/bin`, make sure to include this in your `$PATH`.

To run tests:
```
cargo test
```

To generate html docs to `target/doc/shepherd/index.html`
```
cargo doc
```
