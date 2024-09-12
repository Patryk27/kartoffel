# kartoffel

starter pack for a 🥔 [kartoffels](https://github.com/patryk27/kartoffels) 🥔
bot

## usage

clone this repository, run `./build` and then upload `./kartoffel` into
[the game](https://kartoffels.pwy.io)

(if you're more into terminals, there's `ssh kartoffels.pwy.io`)

note that this repository provides a skeleton robot meant mostly for the in-game
tutorial - it probably won't survive long when deployed onto the battlefield

## caveat emptor

when creating a kartoffel you're implementing a firmware, so you don't have
access to `std` - no `std::fs`, no `println!()` etc. you can communicate through `serial_send()`, though, and you can use various fancy things like `vec` or
`format!()` via the `alloc` crate, just add `extern crate alloc;`

you're given 64 khz cpu and 128 kib of ram, godspeed

## license

cc0 1.0 universal

the person who associated a work with this deed has dedicated the work to the
public domain by waiving all of his or her rights to the work worldwide under
copyright law, including all related and neighboring rights, to the extent
allowed by law.

you can copy, modify, distribute and perform the work, even for commercial
purposes, all without asking permission.
