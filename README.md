# Rust-SDL2 [![Build Status][trav-ci-img]][trav-ci]

Bindings for SDL2 in Rust

# Overview

Rust-SDL2 is a library for talking to the new SDL2.0 libraries from Rust.
Low-level C components are wrapped in Rust code to make them more idiomatic and
abstract away inappropriate manual memory management.

Rust-SDL2 uses the MIT license.

If you want a library compatible with earlier versions of SDL, please see
[here][early-sdl]

# Documentation

* [http://angrylawyer.github.io/rust-sdl2/sdl2/](http://angrylawyer.github.io/rust-sdl2/sdl2/)


## Where are SDL_image, SDL_mixer, and SDL_ttf?

These live outside of the repo.

* https://github.com/xsleonard/rust-sdl2_image
* https://github.com/andelf/rust-sdl2_ttf
* https://github.com/andelf/rust-sdl2_mixer
* https://github.com/andelf/rust-sdl2_gfx
* https://github.com/Limvot/rust-sdl2_net

# Requirements

## Rust

We currently target the latest stable release of Rust.

## *SDL2.0  development libraries*
### Linux
Install these through your favourite package management tool, or via
http://www.libsdl.org/

Ubuntu example:
> sudo apt-get install libsdl2-dev

Fedora example:
> sudo dnf install SDL2-devel

You might also need a C compiler (`gcc`).

### Mac OS X
#### If you are using homebrew
On OSX, it's a good idea to install these via
[homebrew][homebrew].

> brew install sdl2

Then add the following to your `~/.bash_profile` if not already present.

> export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"

##### Otherwise if you are using macports
You can also get sdl2 via `macports`.

> sudo port install libsdl2

Then add the following to your `~/.bash_profile` if not already present.

> export LIBRARY_PATH="$LIBRARY_PATH:/opt/local/lib/"

If you're having issues with either homebrew or macports, [see here][pdev-issue].

#### If you are using the SDL2 framework

You can download and install the SDL2 Mac OS X framework from:
https://www.libsdl.org/download-2.0.php

To make the `sdl2` crate link with the SDL2 framework, you will need to enable
the `use_mac_framework` feature.  To build and test the `sdl2` crate with this
feature, use:

> cargo test --features use_mac_framework

To depend on the `sdl2` crate with this feature enabled, put the following in
your project's `Cargo.toml` file:

```toml
[dependencies.sdl2]
features = ["use_mac_framework"]
version = ...  # Whichever version you are using
```

Alternatively, you can re-export the feature in your package by putting the
following in your `Cargo.toml` file:

```toml
[features]
default = []
use_sdl2_mac_framework = ["sdl2/use_mac_framework"]
```

### Windows (MinGW)

1. Download mingw development libraries from
http://www.libsdl.org/ (SDL2-devel-2.0.x-mingw.tar.gz).
2. Unpack to a folder of your choosing (You can delete it afterwards).
3. Copy all lib files from
    > SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\lib

    to (for Rust 1.6 and above)
    > C:\Program Files\Rust\\**lib**\rustlib\x86_64-pc-windows-gnu\lib

    or to (for Rust versions 1.5 and below)
    > C:\Program Files\Rust\\**bin**\rustlib\x86_64-pc-windows-gnu\lib

    or to your library folder of choice, and ensure you have a system environment variable of
    > LIBRARY_PATH = C:\your\rust\library\folder

	For Multirust Users, this folder will be in
	> C:\Users\{Your Username}\AppData\Local\.multirust\toolchains\{current toolchain}\lib\rustlib\x86_64-pc-windows-gnu\lib

4. Copy SDL2.dll from
    > SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\bin

    into your cargo project, right next to your Cargo.toml.

### Windows (MSVC)

1. Download MSVC development libraries from
http://www.libsdl.org/ (SDL2-devel-2.0.x-VC.zip).
2. Unpack to a folder of your choosing (You can delete it afterwards).
3. Copy both all lib files from
    > SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64

    to (for Rust 1.6 and above)
    > C:\Program Files\Rust\\**lib**\rustlib\x86_64-pc-windows-msvc\lib

    or to (for Rust versions 1.5 and below)
    > C:\Program Files\Rust\\**bin**\rustlib\x86_64-pc-windows-msvc\lib

    or to your library folder of choice, and ensure you have a system environment variable of
    > LIBRARY_PATH = C:\your\rust\library\folder

	For Multirust Users, this folder will be in
	> C:\Users\{Your Username}\AppData\Local\.multirust\toolchains\{current toolchain}\lib\rustlib\x86_64-pc-windows-msvc\lib

4. Copy SDL2.dll from
    > SDL2-devel-2.0.5-VC\SDL2-2.0.5\lib\x64

    into your cargo project, right next to your Cargo.toml.

# Installation

If you're using [cargo][crates] to manage your project, you can
download through Crates.io:

```toml
    [dependencies]
    sdl2 = "0.24"
```

Alternatively, pull it from GitHub

```rust
    [dependencies.sdl2]
    git = "https://github.com/AngryLawyer/rust-sdl2"
```

Otherwise, clone this repo and run [cargo][crates]

> cargo build

# Demo

We have some simple example projects included:

> cargo run --example demo

> cargo run --example audio-whitenoise

# OpenGL

If you want to use OpenGL, you also need the
[gl-rs][gl-rs] package. If you're using
[cargo][crates], just add these lines to your Cargo.toml:

```toml
    [dependencies.gl]
    git = "https://github.com/bjz/gl-rs"
```

Then you need to add this initialization code to establish the
bindings:

```rust
let sdl_context = sdl2::init().unwrap();
let video_subsystem = sdl_context.video().unwrap();

gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
```

Note that these bindings are very raw, and many of the calls will require
unsafe blocks.

# When things go wrong
Rust, and Rust-SDL2, are both still heavily in development, and you may run
into teething issues when using this. Before panicking, check that you're using
the latest version of both Rust and Cargo, check that you've updated Rust-SDL2
to the latest version, and run `cargo clean`. If that fails, please let us know
on the issue tracker.

[trav-ci-img]: https://travis-ci.org/AngryLawyer/rust-sdl2.png?branch=master
[trav-ci]: https://travis-ci.org/AngryLawyer/rust-sdl2
[early-sdl]: https://github.com/brson/rust-sdl
[homebrew]: http://brew.sh/
[crates]: http://crates.io/
[examples]: https://github.com/jdeseno/rs-sdl2-examples
[gl-rs]: https://github.com/bjz/gl-rs
[pdev-issue]: https://github.com/PistonDevelopers/rust-empty/issues/175
