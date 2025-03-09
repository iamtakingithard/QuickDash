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

/// Enum representing each way the appication can fail.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Error {
	/// No errors occured, everything executed correctly.
	NoError,
	/// Parsing of command-line options failed.
	OptionParsingError,
	/// Selected and saved hash lengths differ.
	HashLengthDiffers,
	/// Parsing the hashes file failed.
	HashesFileParsingFailure,
	/// The specified amount of files do not match.
	NFilesDiffer(i32),
}

impl Error {
	/// Get the executable exit value from an `Error` instance.
	pub fn exit_value(&self) -> i32 {
		match *self {
			Error::NoError => 0,
			Error::OptionParsingError => 1,
			Error::HashLengthDiffers => 2,
			Error::HashesFileParsingFailure => 3,
			Error::NFilesDiffer(i) => i + 3,
		}
	}
}

impl From<i32> for Error {
	fn from(i: i32) -> Self {
		match i {
			0 => Error::NoError,
			1 => Error::OptionParsingError,
			2 => Error::HashLengthDiffers,
			3 => Error::HashesFileParsingFailure,
			i => Error::NFilesDiffer(i - 3),
		}
	}
}
