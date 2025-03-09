/* Copyright [2025] [Cerda]
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Tool for making/verifying hashes.
//!
//! # Library doc
//!
//! The library it's self is used in this project as well due it contains all
//! needed functions.
//!
//! ## Data flow
//!
//! Hash verification
//!
//! ```plaintext
//! Options
//! |> create_hashes()
//! |> load_hashes()
//! |> compare_hashes()
//! |> write_hash_comparison_results()
//! ```
//!
//! Hash creation
//!
//! ```plaintext
//! Options
//! |> create_hashes()
//! |> write_hashes()
//! ```
//!
//! # Executable manpage
//!
//! Exit values and possible errors:
//!
//! ```text
//! 1   - option parsing error
//! 2   - hash lengths differ between selected and saved
//! 3   - failed to parse hashes file
//! N+3 - N files didn't match
//! ```
//!
//! ## SYNOPSIS
//!
//! [`QuickDash`](https://github.com/AndreVuillemot160/QuickDash) [OPTIONS] [DIRECTORY]
//!
//! ## DESCRIPTION
//!
//! This is a utility for making/checking hashes with blazing-fast speed. All
//! most well-known hash functions are supported, like MD5, SHA1, SHA2 etc. It's
//! licensed under Apache-2.0 License.
//!
//! ## OPTIONS
//!
//! -a --algorithm &lt;algorithm&gt;
//!
//! ```text
//! Quite simple, select the hash you want. Case-insensitive.
//!
//! Supported algorithms: SHA{1,2-,3-{224,256,384,512}, CRC32, MD5, BLAKE{2B,2S,3}, XXH3, XXHASH64
//! ```
//!
//! -c --create
//!
//! ```text
//! A very simple command. What it does, it creates hashes.
//! If user didn't specified the name it will use, the name of folder with `.hash` extension.
//!
//! And will also fail if the output file exists already and the command `--force` is not presented.
//!
//! Only with `--verify`. Overrides `--verify`.
//! ```
//!
//! -v --verify
//!
//! ```text
//! Verify directory hashes. Used by default.
//!
//! Only with `--create`. Overrides `--create`.
//! ```
//!
//! -d --depth &lt;depth&gt;
//!
//! ```text
//! Set max recursion depth to `depth`. Default: 0.
//!
//! Only with `--recursive`. Overrides `--recursive`.
//! ```
//!
//! -r --recursive
//!
//! ```text
//! Set max recursion depth to infinity. By default it doesn't recurse.
//!
//! Only with `--depth`. Overrides `--depth`.
//! ```
//!
//! --follow-symlinks
//!
//! ```text
//! Recurse down symlinks. Default.
//! ```
//!
//! --no-follow-symlinks
//!
//! ```text
//! Don't recurse down symlinks.
//! ```
//!
//! -i --ignore &lt;filename[,filename2][,filename3][,filenameN]...&gt;...
//!
//! ```text
//! Add filename(s) to ignored files list. Default: not used.
//!
//! The program marks the files that will be ignored. The specified ignored files will not come to the output file.
//!
//! Can be used multiple times.
//! ```
//!
//! --force
//!
//! ```text
//! Rewrite the output file in `--create` mode.
//! ```
//!
//! -j --jobs [jobs]
//!
//! ```text
//! Amount of threads used for hashing. Default: # of CPU threads
//!
//! One thread can hash one file at a time, potentially speeding up hashing
//! up to `jobs` times.
//!
//! No/empty value: # of CPU threads. value = 0: maximum, of u8 (255)
//!
//! ```
//!
//! [DIRECTORY]
//!
//! ```text
//! Directory to create/verify hash for. By default is current workdir.
//! ```
//!
//! ## EXAMPLES
//!
//! `quickdash` [`-v`] [`-f` *infile*]
//!
//! ```text
//! Verify the current directory using the saved hashes.
//!
//! `-v` is not necessary as it's the default.
//!
//! *infile* defaults to "`DIRECTORY`.hash"
//!
//! Example output:
//!   File added: "file_that_hasnt_been_before"
//!   File removed: "file_that_was_originally_here_before_but_not_now"
//!   File ignored: "file_specified_with_ignore_now_or_during_creation"
//!
//!   File "file_that_did_not_change" matches
//!   File "changed_file" doesn't match
//!     Was: foo
//!     Is : bar
//! ```
//!
//! `examples` `-c` [`-f` *outfile*] [`--force`]
//!
//! ```text
//! Create hashes of the current directory tree for later verification.
//!
//! *outfile* defaults to "`DIRECTORY`.hash".
//!
//! Use `--force` to override *outfile*.
//!
//! *outfile* contents:
//!   F013BF0B163785CBB3BE52DE981E069E2B64E1CAC863815AC7BEED63E1734BAE  Cargo.toml
//!   E84E380AEBDA3D98E96267201D61784C3D6FFB128C4D669E6C1D994C7D7BF32B  Cross.toml
//! ```

#![deny(unsafe_code)]
#![allow(clippy::tabs_in_doc_comments)]

mod algorithms;
mod error;
mod hashing;
mod options;

pub mod operations;
pub mod utilities;

pub use crate::{
	algorithms::Algorithm,
	error::Error,
	hashing::*,
	options::{Commands, Mode},
};
