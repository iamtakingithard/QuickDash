![The origins](https://raw.githubusercontent.com/AndreVuillemot160/QuickDash/main/1620228832249.jpg)


# QuickDash [![Rust](https://github.com/AndreVuillemot160/QuickDash/actions/workflows/rust.yml/badge.svg)](https://github.com/AndreVuillemot160/QuickDash/actions/workflows/rust.yml) [![](https://meritbadge.herokuapp.com/quick_dash)](https://crates.io/crates/quick_dash) [![codecov](https://codecov.io/gh/AndreVuillemot160/QuickDash/branch/main/graph/badge.svg?token=YA4NPM8NPJ)](https://codecov.io/gh/AndreVuillemot160/QuickDash)
A modern alternative to QuickSFV using Rust. It's supports BLAKE3 and BLAKE2 hashes, CRC32, MD5, SHA1, SHA2, SHA3, xxHash
If you need [`docs`](https://github.com/AndreVuillemot160/QuickDash/blob/main/DOCS.md) click on the docs.
Mirror: https://git.envs.net/Adrec/QuickDash

## Benchmarks
Benchmarks were performed under Windows 10 with Ryzen 5 1600 with batch scripts that are in project.
For benchmarking the program [`hyperfine`](https://github.com/sharkdp/hyperfine) was used.
It was checking the hashed the source code of the QuickDash.

```
Benchmark #1: quick_dash.exe -a CRC32 --verify -f TEST.sfv
  Time (mean ± σ):      10.7 ms ±   2.9 ms    [User: 12.8 ms, System: 3.2 ms]
  Range (min … max):     9.5 ms …  23.3 ms    233 runs

Benchmark #2: quicksfv.exe TEST.sfv
  Time (mean ± σ):      83.7 ms ±  16.0 ms    [User: 30.9 ms, System: 28.0 ms]
  Range (min … max):    63.8 ms … 117.4 ms    31 runs
```

## Contacts
You are free to get me at [Element/Matrix](https://matrix.to/#/@cerda:envs.net) and in [Discord server](https://discord.gg/cJFekwznKt)

## Building
Well, just go clone the repo, then cd to the cloned repo, and write `cargo build --release`
My builds are built with nigthly version `cargo +nightly build --all-features --release`

## License
This program is licensed under [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/) license.

## Thanks
I would like to say thanks to the [Timo](https://github.com/timokoesters) and /bpg/ community in 4chan, specifically Discord one.
