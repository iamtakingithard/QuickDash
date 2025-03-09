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

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::Algorithm;

#[derive(Parser)]
#[command(
	name = "QuickDash",
	version,
	about,
	long_about = "A modern alternative to QuickSFV using Rust. Made with <3 by Cerda."
)]
pub struct Commands {
	/// Hashing algorithm to use.
	#[arg(value_enum, short, long, default_value = "blake3")]
	pub algorithm: Algorithm,
	/// Max recursion depth. Infinite if None. Default: `0`
	#[arg(short, long)]
	pub depth: Option<usize>,
	/// Whether to recurse down symlinks. Default: `true`
	#[arg(long)]
	pub follow_symlinks: bool,
	/// Files/directories to ignore. Default: none
	#[arg(short, long)]
	pub ignored_files: Vec<String>,
	/// # of threads used for hashing.
	#[arg(short, long, default_value_t = 0)]
	pub jobs: usize,
	/// Whether to verify or create hashes. Default: Verify
	#[command(subcommand)]
	pub command: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
	/// Create a hash file
	Create {
		/// Directory to hash. Default: current directory
		#[arg(default_value = ".")]
		path: PathBuf,
		/// Output filename. Default: `directory_name.hash"`
		#[arg(long)]
		file: Option<PathBuf>,
		#[arg(short, long)]
		force: bool,
	},
	/// Verify a hash file
	Verify {
		/// Directory to verify. Default: current directory
		#[arg(default_value = ".")]
		path: PathBuf,
		/// Input filename. Default: `directory_name.hash`
		#[arg(long)]
		file: Option<PathBuf>,
	},
}
