# pdcurses-sys [![Build status](https://ci.appveyor.com/api/projects/status/7quldtl11lsitu2v?svg=true)](https://ci.appveyor.com/project/ihalila/pdcurses-sys)

pdcurses-sys provides Rust FFI bindings for [PDCurses](http://wmcbrine.com/pdcurses/),
specifically the win32a implementation by [Bill-Gray](https://github.com/Bill-Gray/PDCurses).

## Requirements

A native C compiler that [gcc-rs](http://alexcrichton.com/gcc-rs/gcc/index.html)
can use to compile PDCurses.

## Usage

Cargo.toml
```toml
[dependencies]
pdcurses = "0.1"
```

## License

Licensed under the MIT license, see [LICENSE.md](LICENSE.md)
