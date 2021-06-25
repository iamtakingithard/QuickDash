# CHANGELOG (dates are in fromat YYYY-MM-DD)


## 0.5.3 (2021-06-25)

Added XXH32
Windows builds are compressed with UPX
Added code coverage, and wrote some tests
From this version all builds are built via GitHub actions.

## 0.5.1 (2021-06-21)

Ability to read both `hash - filename` and vice versa

## 0.5.0 (2021-06-19)

Switching to Futures 0.3

## 0.4.0 and 0.4.1 (2021-06-08)

Adding support for xxHash.

## 0.3.8 (2021-06-04)

Switched all crates to RustCrypto maintained ones, also switched to Rust maintained sha1 crate.
Added SHA-2,SHA-3, Whirlpool support.

## 0.3.6 (2021-06-01)

Switched CRC crate, to crc32fast, which removed the support of crc64 but made crc32 far more faster

## 0.3.5 (2020-05-31)

Program, can read lowercase hashes.
