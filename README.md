# nyaa-tui

small project to play around with rust

currently still very much so WIP

## Notes

- all popups will be blocking. you can't switch tabs until you closed a popup
- nyaa_entries are sorted by seeders
- downloads follow the first-in-first-out methodology


### Current Features:

- Fetch and extract data from nyaa.si
- output the list of fetched entries
</br></br>  

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

- crossterm ([crates.io](https://crates.io/crates/crossterm) | [github](https://github.com/crossterm-rs/crossterm))
- ratatui ([crates.io](https://crates.io/crates/ratatui) | [github](https://github.com/ratatui-org/ratatui))
- reqwest ([crates.io](https://crates.io/crates/reqwest) | [github](https://github.com/seanmonstar/reqwest))
- reqwest-retry ([crates.io](https://crates.io/crates/reqwest-retry) | [github](https://github.com/TrueLayer/reqwest-middleware))
- reqwest-middleware ([crates.io](https://crates.io/crates/reqwest-middleware) | [github](https://github.com/TrueLayer/reqwest-middleware))
- tokio ([crates.io](https://crates.io/crates/tokio) | [github](https://github.com/tokio-rs/tokio))
- unhtml ([crates.io](https://crates.io/crates/unhtml) | [github](https://github.com/Hexilee/unhtml.rs))

# Dependencies

On Linux: 
- openssl

# Disclaimer 

[read the disclaimer here](https://github.com/woolw/nyaa-tui/blob/master/DISCLAIMER.md)