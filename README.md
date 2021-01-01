# ht

**DEPRECATED**: This project has been superceded by [decaale/ht](https://github.com/ducaale/ht).

ht is a user-friendly alternative to curl, inspired by
[HTTPie](https://github.com/jakubroztocil/httpie).

**NOTE**: This is a young, side-project just getting off the ground. Issues
and Pull Requests are welcome!

## Installation

Currently, the only way to install ht is via cargo:

```bash
cargo install ht
```

## Usage

Currently, only `GET` is supported:

```bash
ht 'https://httpbin.org/get?show_env=1&foo=baz&foo=bar'
```

## License

This software is licensed under the Apache 2.0 License, see the `LICENSE`
file for details.
