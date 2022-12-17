![The origins](https://raw.githubusercontent.com/iamtakingithard/QuickDash/main/1620228832249.jpg)


# QuickDash [![Rust](https://github.com/iamtakingithard/QuickDash/actions/workflows/rust.yml/badge.svg)](https://github.com/iamtakingithard/QuickDash/actions/workflows/rust.yml) [![](https://meritbadge.herokuapp.com/quickdash)](https://crates.io/crates/quickdash) [![codecov](https://codecov.io/gh/iamtakingithard/QuickDash/branch/main/graph/badge.svg?token=YA4NPM8NPJ)](https://codecov.io/gh/iamtakingithard/QuickDash)
A modern alternative to QuickSFV using Rust. It's supports BLAKE3 and BLAKE2 hashes, CRC32, MD5, SHA1, SHA2, SHA3, xxHash

Note: the old name `quick_dash` is no longer in use, if anyone wants it feel free to take it on crates.io

## Benchmarks
Benchmarks were performed under Windows 10 with Ryzen 5 1600.
For benchmarking the program [`hyperfine`](https://github.com/sharkdp/hyperfine) was used.
It was checking the hashed the source code of the QuickDash.

```
Benchmark #1: quickdash.exe -a CRC32 --verify -f TEST.sfv
  Time (mean ± σ):      10.7 ms ±   2.9 ms    [User: 12.8 ms, System: 3.2 ms]
  Range (min … max):     9.5 ms …  23.3 ms    233 runs

Benchmark #2: quicksfv.exe TEST.sfv
  Time (mean ± σ):      83.7 ms ±  16.0 ms    [User: 30.9 ms, System: 28.0 ms]
  Range (min … max):    63.8 ms … 117.4 ms    31 runs
```

## Install
There are two ways of doing that. You can
A) Get a binary from crates.io with command `cargo install quickdash`
B) Get a already compiled binary from GitHub, which features Windows, Mac, Linux builds.

## Building
Well, just download the source code, then go to the cloned repo, and write `cargo build --release`

## License
This program is licensed under [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/) license.

## Thanks
I would like to say thanks to the [Timo](https://github.com/timokoesters) and all future contributors to this project.
