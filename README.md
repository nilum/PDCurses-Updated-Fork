# pdcurses-sys [![Build status](https://ci.appveyor.com/api/projects/status/7quldtl11lsitu2v?svg=true)](https://ci.appveyor.com/project/ihalila/pdcurses-sys) [![Crates.io](https://img.shields.io/crates/v/pdcurses-sys.svg)](https://crates.io/crates/pdcurses-sys)

pdcurses-sys provides Rust FFI bindings for [PDCurses](http://wmcbrine.com/pdcurses/),
specifically the win32a implementation by [Bill-Gray](https://github.com/Bill-Gray/PDCurses).

## Requirements

A native C compiler that [gcc-rs](http://alexcrichton.com/gcc-rs/gcc/index.html)
can use to compile PDCurses.

If you've installed the default Rust package for Windows, it will use the GNU
ABI and in this case I recommend installing [MSYS2](https://msys2.github.io/),
it makes it easy to have an up-to-date gcc that works on Windows along with
other useful tools.

If you're using the MSVC ABI you need to have Visual Studio installed so that
cl.exe is available.

## Usage

Cargo.toml
```toml
[dependencies]
pdcurses = "0.3"
```

## License

Licensed under the MIT license, see [LICENSE.md](LICENSE.md)
