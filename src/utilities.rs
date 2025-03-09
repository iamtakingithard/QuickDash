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

//! Module containing various utility functions

use std::path::Path;

/// Merges two `Vec`s.
///
/// # Examples
///
/// ```
/// let vec1 = vec![0];
/// let vec2 = vec![1];
///
/// assert_eq!(quickdash::utilities::vec_merge(vec1, vec2), vec![0, 1]);
/// ```
pub fn vec_merge<T>(mut lhs: Vec<T>, rhs: Vec<T>) -> Vec<T> {
	lhs.extend(rhs);
	lhs
}

/// Create a string consisting of `n` repetitions of `what`.
///
/// # Examples
///
/// ```
/// assert_eq!(
/// 	quickdash::utilities::mul_str("LOL! ", 3),
/// 	"LOL! LOL! LOL! ".to_string()
/// );
/// ```
pub fn mul_str(what: &str, n: usize) -> String {
	what.repeat(n)
}

/// Create a user-usable path to `what` from `prefix`.
///
/// # Examples
///
/// ```
/// # use std::path::Path;
/// assert_eq!(
/// 	quickdash::utilities::relative_name(Path::new("/usr"), Path::new("/usr/bin/quickdash")),
/// 	"bin/quickdash".to_string()
/// );
/// ```
pub fn relative_name(prefix: &Path, what: &Path) -> String {
	what.strip_prefix(prefix)
		.unwrap()
		.to_str()
		.unwrap()
		.replace('\\', "/")
}
