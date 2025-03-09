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

use std::io::Write;

use super::{CompareError, CompareFileResult, CompareResult};
use crate::{Error, utilities::mul_str};

/// Write hash comparison results to the output streams in a human-consumable
/// format
pub fn write_hash_comparison_results<Wo: Write, We: Write>(
	output: &mut Wo,
	error: &mut We,
	results: Result<(Vec<CompareResult>, Vec<CompareFileResult>), CompareError>,
) -> Error {
	let result = match results {
		Ok((mut compare_results, mut file_compare_results)) => {
			compare_results.sort();
			file_compare_results.sort();

			for res in &compare_results {
				match *res {
					CompareResult::FileAdded(ref file) => {
						write_compare_result(output, "File added: ", file)
					}
					CompareResult::FileRemoved(ref file) => {
						write_compare_result(output, "File removed: ", file)
					}
					CompareResult::FileIgnored(ref file) => {
						write_compare_result(output, "File ignored, skipping: ", file)
					}
				}
			}

			if file_compare_results.is_empty() && compare_results.is_empty() {
				writeln!(output, "No files left to verify").expect("io err");
				Error::NoError
			} else if file_compare_results.is_empty() {
				writeln!(output, "No files to verify").expect("io err");
				Error::NoError
			} else {
				if !compare_results.is_empty() {
					writeln!(output).unwrap();
				}

				let mut differed_n = 0;
				for fres in &file_compare_results {
					match *fres {
						CompareFileResult::FileMatches(ref file) => {
							write_file_result_match(output, file)
						}
						CompareFileResult::FileDiffers {
							ref file,
							ref was_hash,
							ref new_hash,
						} => {
							write_file_result_diff(output, file, was_hash, new_hash);
							differed_n += 1;
						}
					}
				}

				match differed_n {
					0 => Error::NoError,
					n => Error::NFilesDiffer(n),
				}
			}
		}
		Err(CompareError::HashLengthDiffers {
			previous_len,
			current_len,
		}) => {
			let previous_len_len = format!("{}", previous_len).len();
			let current_len_len = format!("{}", current_len).len();

			if previous_len_len + current_len_len + 47 <= 80 {
				writeln!(
					error,
					"Hash lengths do not match; selected: {}, loaded: {}",
					current_len, previous_len
				)
				.unwrap();
			} else {
				writeln!(error, "Hash lengths do not match;").unwrap();
				if previous_len_len + current_len_len + 20 <= 80 {
					writeln!(error, "selected: {}, loaded: {}", current_len, previous_len).unwrap();
				} else {
					writeln!(error, "Selected: {}", current_len).unwrap();
					writeln!(error, "Loaded  : {}", previous_len).unwrap();
				}
			}

			Error::HashLengthDiffers
		}
	};

	output.flush().unwrap();
	error.flush().unwrap();

	result
}

fn write_compare_result<W: Write>(out: &mut W, pre: &str, fname: &str) {
	write_result(out, pre, fname, 2, true)
}

fn write_result<W: Write>(out: &mut W, pre: &str, fname: &str, fname_indent: usize, quote: bool) {
	if pre.len() + quote as usize + fname.len() + quote as usize <= 80 {
		let quote_s = if quote { "\"" } else { "" };
		writeln!(out, "{}{2}{}{2}", pre, fname, quote_s).unwrap();
	} else {
		writeln!(out, "{}", pre).unwrap();
		if fname.len() <= 80 - fname_indent {
			writeln!(out, "  {}", fname).unwrap();
		} else {
			let indent = mul_str(" ", fname_indent);
			for fname_chunk in fname
				.chars()
				.collect::<Vec<_>>()
				.chunks(80 - fname_indent)
				.map(|cc| cc.iter().cloned().collect::<String>())
			{
				writeln!(out, "{}{}", indent, fname_chunk).unwrap();
			}
		}
	}
}

fn write_file_result_match<W: Write>(out: &mut W, fname: &str) {
	if 15 + fname.len() <= 80 {
		writeln!(out, "File \"{}\" matches", fname).unwrap();
	} else {
		write_compare_result(out, "File matches: ", fname);
	}
}

fn write_file_result_diff<W: Write>(out: &mut W, fname: &str, lhash: &str, chash: &str) {
	if 21 + fname.len() <= 80 {
		writeln!(out, "File \"{}\" doesn't match", fname).unwrap();
	} else {
		write_result(out, "File doesn't match: ", fname, 4, true);
	}

	write_result(out, "  Was: ", lhash, 4, false);
	write_result(out, "  Is : ", chash, 4, false);
}
