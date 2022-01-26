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

use std::{
	io::{stderr, stdout},
	process::exit,
};

fn main() {
	let result = actual_main();
	exit(result);
}

fn actual_main() -> i32 {
	let opts = quickdash::Options::parse();

	let hashes = quickdash::operations::create_hashes(
		&opts.dir,
		opts.ignored_files,
		opts.algorithm,
		opts.depth,
		opts.follow_symlinks,
		opts.jobs,
	);
	if opts.verify {
		// Progress bar separator
		println!();
		// PS I know it's kinda awful and all. But still cool.
		println!("  █████   █    ██  ██▓ ▄████▄   ██ ▄█▀▓█████▄  ▄▄▄        ██████  ██░ ██ ");
		println!("▒██▓  ██▒ ██  ▓██▒▓██▒▒██▀ ▀█   ██▄█▒ ▒██▀ ██▌▒████▄    ▒██    ▒ ▓██░ ██▒");
		println!("▒██▒  ██░▓██  ▒██░▒██▒▒▓█    ▄ ▓███▄░ ░██   █▌▒██  ▀█▄  ░ ▓██▄   ▒██▀▀██░");
		println!("░██  █▀ ░▓▓█  ░██░░██░▒▓▓▄ ▄██▒▓██ █▄ ░▓█▄   ▌░██▄▄▄▄██   ▒   ██▒░▓█ ░██ ");
		println!("░▒███▒█▄ ▒▒█████▓ ░██░▒ ▓███▀ ░▒██▒ █▄░▒████▓  ▓█   ▓██▒▒██████▒▒░▓█▒░██▓");
		println!("░░ ▒▒░ ▒ ░▒▓▒ ▒ ▒ ░▓  ░ ░▒ ▒  ░▒ ▒▒ ▓▒ ▒▒▓  ▒  ▒▒   ▓▒█░▒ ▒▓▒ ▒ ░ ▒ ░░▒░▒");
		println!(" ░ ▒░  ░ ░░▒░ ░ ░  ▒ ░  ░  ▒   ░ ░▒ ▒░ ░ ▒  ▒   ▒   ▒▒ ░░ ░▒  ░ ░ ▒ ░▒░ ░");
		println!("   ░   ░  ░░░ ░ ░  ▒ ░░        ░ ░░ ░  ░ ░  ░   ░   ▒   ░  ░  ░   ░  ░░ ░");
		println!("     ░       ░      ░  ░ ░      ░  ░      ░          ░  ░      ░   ░  ░  ░");
		println!();
		println!("Made with <3 by Cerda. Repo: https://github.com/AndreVuillemot160/QuickDash/");
		println!();

		match quickdash::operations::read_hashes(&opts.file) {
			Ok(loaded_hashes) => {
				let compare_result =
					quickdash::operations::compare_hashes(&opts.file.0, hashes, loaded_hashes);
				quickdash::operations::write_hash_comparison_results(
					&mut stdout(),
					&mut stderr(),
					compare_result,
				)
			}
			Err(rval) => rval,
		}
		.exit_value()
	} else {
		quickdash::operations::write_hashes(&opts.file, opts.algorithm, hashes);
		0
	}
}
