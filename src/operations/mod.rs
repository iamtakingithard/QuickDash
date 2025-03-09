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

//! Main functions doing actual work.
//!
//!
//! Use `create_hashes()` to prepare the hashes for a path.
//!
//! Then use `write_hashes()` to save it to disk, or `read_hashes()` to get the
//! saved hashes, them with `compare_hashes()` and print them with
//! `write_hash_comparison_results()`.

mod compare;
mod write;

use std::{
	collections::BTreeMap,
	fs::File,
	io::{BufRead, BufReader, Write},
	path::Path,
	time::Duration,
};

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use rayon::{
	ThreadPoolBuilder,
	iter::{IntoParallelRefIterator, ParallelIterator},
};
use regex::Regex;
use tabwriter::TabWriter;
use walkdir::{DirEntry, WalkDir};

pub use self::{compare::*, write::*};
use crate::{
	Algorithm, Error, hash_file,
	utilities::{mul_str, relative_name},
};

static SPINNER_STRINGS: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Create subpath->hash mappings for a given path using a given algorithm up to
/// a given depth.
pub fn create_hashes(
	path: &Path,
	ignored_files: Vec<String>,
	algo: Algorithm,
	depth: Option<usize>,
	follow_symlinks: bool,
	jobs: usize,
) -> BTreeMap<String, String> {
	let mut walkdir = WalkDir::new(path).follow_links(follow_symlinks);
	if let Some(depth) = depth {
		walkdir = walkdir.max_depth(depth + 1);
	}

	let pb_style = ProgressStyle::default_bar()
		.template("{prefix:.bold.dim} {spinner} {wide_bar} {pos:>7}/{len:7} ETA: {eta} - {msg}")
		.unwrap()
		.tick_strings(&SPINNER_STRINGS);

	let pb = ProgressBar::new_spinner();
	pb.set_style(pb_style);

	ThreadPoolBuilder::new()
		.num_threads(jobs)
		.build_global()
		.unwrap();

	let mut hashes = BTreeMap::new();

	pb.enable_steady_tick(Duration::from_millis(80));
	pb.set_message("Finding files to hash...");
	let mut files: Vec<DirEntry> = walkdir
		.into_iter()
		.filter_entry(|e: &walkdir::DirEntry| {
			let filename = relative_name(path, e.path());
			match (ignored_files.contains(&filename), e.file_type().is_file()) {
				(true, true) => {
					hashes.insert(mul_str("-", algo.hexlen()), filename);
					false
				}
				(true, false) => false,
				_ => true,
			}
		})
		.flatten()
		.filter(|e| e.file_type().is_file())
		.collect();

	optimize_file_order(&mut files);

	pb.reset();
	pb.set_length(files.len() as u64);
	pb.set_message("Hashing files...");

	let mut result: BTreeMap<String, String> = files
		.par_iter()
		.progress_with(pb)
		.map(|e| {
			let value = hash_file(algo, e.path());
			let filename = relative_name(path, e.path());
			(filename, value)
		})
		.collect();
	hashes.append(&mut result);
	hashes
}

#[cfg(target_os = "linux")]
fn optimize_file_order(dirs: &mut [DirEntry]) {
	use walkdir::DirEntryExt;
	dirs.sort_by(|a, b| {
		let a_inode = a.ino();
		let b_inode = b.ino();
		a_inode.cmp(&b_inode)
	});
}

#[cfg(not(target_os = "linux"))]
fn optimize_file_order(_dirs: &mut [DirEntry]) {}

/// Serialise the specified hashes to the specified output file.
pub fn write_hashes(out_file: &Path, algo: Algorithm, mut hashes: BTreeMap<String, String>) -> i32 {
	let file = File::create(&out_file).unwrap();
	let mut out = TabWriter::new(file);

	hashes.insert(
		out_file.to_string_lossy().to_string(),
		mul_str("-", algo.hexlen()),
	);
	for (fname, hash) in hashes {
		writeln!(&mut out, "{}  {}", hash, fname).unwrap();
	}

	out.flush().expect("Failed to flush output file");
	0
}

/// Read uppercased hashes with `write_hashes()` from the specified path or fail
/// with line numbers not matching pattern.
pub fn read_hashes(file: &Path) -> Result<BTreeMap<String, String>, Error> {
	let mut hashes = BTreeMap::new();

	let in_file = BufReader::new(File::open(&file).unwrap());
	for line in in_file.lines().map(Result::unwrap) {
		try_contains(&line, &mut hashes)?;
	}

	Ok(hashes)
}

fn try_contains(line: &str, hashes: &mut BTreeMap<String, String>) -> Result<(), Error> {
	if line.is_empty() {
		return Err(Error::HashesFileParsingFailure);
	}

	static LINE_RGX1: Lazy<Regex> =
		Lazy::new(|| Regex::new(r"(?i)^([[:xdigit:]-]+)\s{2,}(.+?)$").unwrap());

	static LINE_RGX2: Lazy<Regex> =
		Lazy::new(|| Regex::new(r"(?i)^(.+?)\t{0,}\s{1,}([[:xdigit:]-]+)$").unwrap());

	if let Some(captures) = LINE_RGX1.captures(line) {
		hashes.insert(captures[2].to_string(), captures[1].to_uppercase());
		return Ok(());
	}
	if let Some(captures) = LINE_RGX2.captures(line) {
		hashes.insert(captures[1].to_string(), captures[2].to_uppercase());
		return Ok(());
	}
	Err(Error::HashesFileParsingFailure)
}
