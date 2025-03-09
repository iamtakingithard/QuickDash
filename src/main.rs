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

use std::{
	fs::remove_file,
	io::{stderr, stdout},
	path::{Path, PathBuf},
	process::exit,
};

use clap::Parser;
use quickdash::{Commands, Mode};

const BANNER: [&str; 13] = [
	"",
	// PS I know it's kinda awful and all. But still cool.
	"  █████   █    ██  ██▓ ▄████▄   ██ ▄█▀▓█████▄  ▄▄▄        ██████  ██░ ██ ",
	"▒██▓  ██▒ ██  ▓██▒▓██▒▒██▀ ▀█   ██▄█▒ ▒██▀ ██▌▒████▄    ▒██    ▒ ▓██░ ██▒",
	"▒██▒  ██░▓██  ▒██░▒██▒▒▓█    ▄ ▓███▄░ ░██   █▌▒██  ▀█▄  ░ ▓██▄   ▒██▀▀██░",
	"░██  █▀ ░▓▓█  ░██░░██░▒▓▓▄ ▄██▒▓██ █▄ ░▓█▄   ▌░██▄▄▄▄██   ▒   ██▒░▓█ ░██ ",
	"░▒███▒█▄ ▒▒█████▓ ░██░▒ ▓███▀ ░▒██▒ █▄░▒████▓  ▓█   ▓██▒▒██████▒▒░▓█▒░██▓",
	"░░ ▒▒░ ▒ ░▒▓▒ ▒ ▒ ░▓  ░ ░▒ ▒  ░▒ ▒▒ ▓▒ ▒▒▓  ▒  ▒▒   ▓▒█░▒ ▒▓▒ ▒ ░ ▒ ░░▒░▒",
	" ░ ▒░  ░ ░░▒░ ░ ░  ▒ ░  ░  ▒   ░ ░▒ ▒░ ░ ▒  ▒   ▒   ▒▒ ░░ ░▒  ░ ░ ▒ ░▒░ ░",
	"   ░   ░  ░░░ ░ ░  ▒ ░░        ░ ░░ ░  ░ ░  ░   ░   ▒   ░  ░  ░   ░  ░░ ░",
	"     ░       ░      ░  ░ ░      ░  ░      ░          ░  ░      ░   ░  ░  ░",
	"",
	"Made with <3  by Cerda. Repo: https://github.com/iamtakingithard/QuickDash",
	"",
];

fn main() {
	let result = actual_main();
	exit(result);
}

fn actual_main() -> i32 {
	let opts = Commands::parse();

	BANNER.iter().for_each(|line| println!("{}", line));

	match opts.command {
		Mode::Create { path, file, force } => {
			let file = file.unwrap_or_else(|| default_file(&path));
			match (force, Path::new(&file).exists()) {
				(true, _) | (_, false) => {
					// if this fails, it probably didn't exist
					let _ = remove_file(&file);

					let hashes = quickdash::operations::create_hashes(
						&path,
						opts.ignored_files,
						opts.algorithm,
						opts.depth,
						opts.follow_symlinks,
						opts.jobs,
					);
					quickdash::operations::write_hashes(&file, opts.algorithm, hashes)
				}
				(false, true) => {
					eprintln!("File already exists. Use --force to overwrite.");
					1
				}
			}
		}
		Mode::Verify { path, file } => {
			let hashes = quickdash::operations::create_hashes(
				&path,
				opts.ignored_files,
				opts.algorithm,
				opts.depth,
				opts.follow_symlinks,
				opts.jobs,
			);
			let file = file.unwrap_or_else(|| default_file(&path));
			match quickdash::operations::read_hashes(&file) {
				Ok(loaded_hashes) => {
					let compare_result =
						quickdash::operations::compare_hashes(&file, hashes, loaded_hashes);
					quickdash::operations::write_hash_comparison_results(
						&mut stdout(),
						&mut stderr(),
						compare_result,
					)
				}
				Err(rval) => rval,
			}
			.exit_value()
		}
	}
}

fn default_file(path: &Path) -> PathBuf {
	let parent = path.file_stem().expect("Could not get directory name");
	path.join(parent).with_extension("hash")
}
