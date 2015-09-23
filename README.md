# Shouji
---
rust interface for consul

#### disclaimer: this was a coding exercise to learn rust

**note: built on rust nightly**

    rustc --version
    rustc 1.5.0-nightly (b2f379cdc 2015-09-23)

Highly recommend using [multirust](https://github.com/brson/multirust) for installing and managing rust installations.

### Install & Running

    $ git clone https://github.com/jkordish/shouji.git
    $ cd Shouji
    $ cargo build --release
    $ ./target/release/shouji get key/blah

#### Features
Supports:
  * get
  * put
  * list
