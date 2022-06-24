# rustdoc-index

[![crates.io](https://img.shields.io/crates/v/rustdoc-index)](https://crates.io/crates/rustdoc-index) [![docs.rs](https://docs.rs/rustdoc-index/badge.svg)](https://docs.rs/rustdoc-index/) ![](https://github.com/octaltree/rustdoc-index/workflows/Build/badge.svg)

A tool to quickly find the relevant item from rustdoc.

## Installation
$ cargo install rustdoc-index
$ rustup component add rust-docs

It requires Rust 1.53 that is now avaible on stable.

## Usage
```sh
$ cargo listdoc
std::prelude	mod
std::prelude::v1	mod
std::prelude::rust_2015	mod
std::prelude::rust_2018	mod
std::prelude::rust_2021	mod
std::f32	mod
std::f32::consts	mod
std::f32::consts::PI	constant
std::f32::consts::TAU	constant
std::f32::consts::FRAC_PI_2	constant
...
```

```sh
$ cargo listdoc location "std::f32::consts::PI	constant"
file:///home/octaltree/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/html/std/f32/consts/constant.PI.html
```

### With [junegunn/fzf](https://github.com/junegunn/fzf)
```sh
cargo listdoc | fzf --bind 'ctrl-i:execute(cargo listdoc location {}| xargs firefox)'
```
![](https://user-images.githubusercontent.com/7942952/113164022-b2016280-927b-11eb-85fa-0870b817b7cd.gif)

## Vim integration
[denite](https://github.com/Shougo/denite.nvim) source is available.

### Installation
This is how to install using [dein](https://github.com/Shougo/dein.vim).
```toml
[[plugins]]
repo = 'octaltree/rustdoc-index'
build = 'make denite'
```

### Usage
```vim
:Denite rustdoc-index
```

This is highly inspired by [rhysd/rust-doc.vim](https://github.com/rhysd/rust-doc.vim). If you need the unite interface, please use that one.
