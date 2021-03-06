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

use std::{collections::BTreeMap, path::Path};

use crate::utilities::{mul_str, vec_merge};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompareResult {
	FileAdded(String),
	FileRemoved(String),
	FileIgnored(String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompareFileResult {
	FileMatches(String),
	FileDiffers {
		file: String,
		was_hash: String,
		new_hash: String,
	},
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum CompareError {
	HashLengthDiffers {
		previous_len: usize,
		current_len: usize,
	},
}

/// Compare two provided hashes
pub fn compare_hashes(
	out_file: &Path,
	mut current_hashes: BTreeMap<String, String>,
	mut loaded_hashes: BTreeMap<String, String>,
) -> Result<(Vec<CompareResult>, Vec<CompareFileResult>), CompareError> {
	let current_hashes_value_len = current_hashes.iter().next().unwrap().1.len();
	let loaded_hashes_value_len = loaded_hashes.iter().next().unwrap().1.len();
	if current_hashes_value_len != loaded_hashes_value_len {
		return Err(CompareError::HashLengthDiffers {
			previous_len: loaded_hashes_value_len,
			current_len: current_hashes_value_len,
		});
	}
	let placeholder_value = mul_str("-", current_hashes_value_len);
	let mut file_compare_results = Vec::new();

	let key = out_file.to_string_lossy().to_string();
	current_hashes.remove(&key);
	loaded_hashes.remove(&key);

	let remove_results = process_ignores(
		|key, _, other| !other.contains_key(key),
		CompareResult::FileAdded,
		CompareResult::FileRemoved,
		&mut current_hashes,
		&mut loaded_hashes,
	);
	let ignore_results = process_ignores(
		|_, value, _| *value == placeholder_value,
		CompareResult::FileIgnored,
		CompareResult::FileIgnored,
		&mut current_hashes,
		&mut loaded_hashes,
	);

	// By this point both hashes have the same keysets
	assert_eq!(current_hashes.len(), loaded_hashes.len());

	if !current_hashes.is_empty() {
		for (key, loaded_value) in loaded_hashes {
			let current_value = &current_hashes[&key];
			if *current_value == loaded_value {
				file_compare_results.push(CompareFileResult::FileMatches(key));
			} else {
				file_compare_results.push(CompareFileResult::FileDiffers {
					file: key,
					was_hash: loaded_value,
					new_hash: current_value.clone(),
				});
			}
		}
	}

	Ok((
		vec_merge(remove_results, ignore_results),
		file_compare_results,
	))
}

fn process_ignores<F, Rc, Rl>(
	f: F,
	cres: Rc,
	lres: Rl,
	ch: &mut BTreeMap<String, String>,
	lh: &mut BTreeMap<String, String>,
) -> Vec<CompareResult>
where
	F: Fn(&str, &str, &BTreeMap<String, String>) -> bool,
	Rc: Fn(String) -> CompareResult,
	Rl: Fn(String) -> CompareResult,
{
	let mut results = Vec::new();
	let mut keys_to_remove = Vec::new();

	process_ignores_iter(&f, &cres, ch, lh, &mut keys_to_remove, &mut results);
	process_ignores_iter(&f, &lres, lh, ch, &mut keys_to_remove, &mut results);

	for key in keys_to_remove {
		ch.remove(&key);
		lh.remove(&key);
	}

	results
}

fn process_ignores_iter<F, R>(
	f: &F,
	res: &R,
	curr: &BTreeMap<String, String>,
	other: &BTreeMap<String, String>,
	keys_to_remove: &mut Vec<String>,
	results: &mut Vec<CompareResult>,
) where
	F: Fn(&str, &str, &BTreeMap<String, String>) -> bool,
	R: Fn(String) -> CompareResult,
{
	for (key, value) in curr {
		if f(key, value, other) {
			results.push(res(key.clone()));
			keys_to_remove.push(key.clone());
		}
	}
}
