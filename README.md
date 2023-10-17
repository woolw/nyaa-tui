# nyaa-tui

Small scraping project to play around with rust.  
Scrapes nyaa.si and allows you to download files from there using aria2.  
only tested for the linux operating system.

## Install

<details><summary>From source</summary>

This requires you to have cargo set-up on your system.  
Either use your package manager to install `rustup`, or follow the official rust [Getting started](https://www.rust-lang.org/learn/get-started) 

```
git clone https://github.com/woolw/nyaa-tui.git
cd nyaa-tui/
cargo run --release
```
</details>

<details><summary>Binary</summary>

You can find the latest binary [here](https://github.com/woolw/nyaa-tui/releases/latest).  
After you downloaded the binary, you can execute it from the terminal:
```
chmod +x nyaa-tui
./nyaa-tui
```

Or make it executable from everywhere:
```
chmod +x nyaa-tui
sudo cp nyaa-tui /usr/local/bin/
```
</details>

<details><summary>Package</summary>

soon
</details>

## Uninstall

<details><summary>details</summary>

- From source  
Just delete the cloned folder.

- Binary
```
sudo rm /usr/local/bin/nyaa-tui
```
</details>

## Crates used

- crossterm ([crates.io](https://crates.io/crates/crossterm) | [github](https://github.com/crossterm-rs/crossterm))
- ratatui ([crates.io](https://crates.io/crates/ratatui) | [github](https://github.com/ratatui-org/ratatui))
- reqwest ([crates.io](https://crates.io/crates/reqwest) | [github](https://github.com/seanmonstar/reqwest))
- reqwest-retry ([crates.io](https://crates.io/crates/reqwest-retry) | [github](https://github.com/TrueLayer/reqwest-middleware))
- reqwest-middleware ([crates.io](https://crates.io/crates/reqwest-middleware) | [github](https://github.com/TrueLayer/reqwest-middleware))
- tokio ([crates.io](https://crates.io/crates/tokio) | [github](https://github.com/tokio-rs/tokio))
- unhtml ([crates.io](https://crates.io/crates/unhtml) | [github](https://github.com/Hexilee/unhtml.rs))

## Dependencies

- openssl
- aria2

## Disclaimer 

[read the disclaimer here](https://github.com/woolw/nyaa-tui/blob/master/DISCLAIMER.md)