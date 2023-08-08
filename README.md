# nyaa-tui

small project to learn rust, reqwest, tokio

currently still very much so WIP

### Current Features:

- Fetch and extract data from nyaa.si
- print the data to console

# Setup

## Source
1. clone the repo
```
git clone https://github.com/woolw/nyaa-tui.git
```
2. cd into the cloned repo
```
cd nyaa-tui/
```
3. cargo run the project
```
cargo run --release
```

## Package

none so far

# Crates used

- unhtml ([crates.io](https://crates.io/crates/unhtml) | [github](https://github.com/Hexilee/unhtml.rs))
- tokio ([crates.io](https://crates.io/crates/tokio) | [github](https://github.com/tokio-rs/tokio))
- reqwest ([crates.io](https://crates.io/crates/reqwest) | [github](https://github.com/seanmonstar/reqwest))
- reqwest-retry ([crates.io](https://crates.io/crates/reqwest-retry) | [github](https://github.com/TrueLayer/reqwest-middleware))
- reqwest-middleware ([crates.io](https://crates.io/crates/reqwest-middleware) | [github](https://github.com/TrueLayer/reqwest-middleware))
- tui ([crates.io](https://crates.io/crates/tui) | [github](https://github.com/fdehau/tui-rs))

# Dependencies

On Linux: 
- openssl

# Disclaimer 

[read the disclaimer here](https://github.com/woolw/nyaa-tui/blob/master/DISCLAIMER.md)