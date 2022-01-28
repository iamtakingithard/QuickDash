/* Copyright [2021] [Cerda]
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

use std::path::PathBuf;

use crate::Algorithm;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
	name = "QuickDash",
	version,
	about,
	long_about = "A modern alternative to QuickSFV using Rust. Made with <3 by Cerda."
)]
pub struct Commands {
	/// Hashing algorithm to use.
	#[clap(arg_enum, short, long, default_value = "blake3")]
	pub algorithm: Algorithm,
	/// Max recursion depth. Infinite if None. Default: `0`
	#[clap(short, long)]
	pub depth: Option<usize>,
	/// Whether to recurse down symlinks. Default: `true`
	#[clap(long)]
	pub follow_symlinks: bool,
	/// Files/directories to ignore. Default: none
	#[clap(short, long)]
	pub ignored_files: Vec<String>,
	/// # of threads used for hashing.
	#[clap(short, long, default_value_t = 0)]
	pub jobs: usize,
	/// Whether to verify or create hashes. Default: Verify
	#[clap(subcommand)]
	pub command: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
	Create {
		/// Directory to hash. Default: current directory
		#[clap(default_value = ".")]
		path: PathBuf,
		/// Output filename. Default: `directory_name.hash"`
		#[clap(long)]
		file: Option<PathBuf>,
		#[clap(short, long)]
		force: bool,
	},
	Verify {
		/// Directory to verify. Default: current directory
		#[clap(default_value = ".")]
		path: PathBuf,
		/// Input filename. Default: `directory_name.hash"`
		#[clap(long)]
		file: Option<PathBuf>,
	},
}

/// Representation of the application's all configurable values.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Options {}
