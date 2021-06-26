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

/// Enum representing each way the appication can fail.
use std::fmt::{self, Formatter};

/// Enum representing each way the appication can fail.
#[derive(Debug)]
pub enum Error<'err> {
    /// Parsing of command-line options failed.
    ArgumentParsingError(clap::Error),
    /// Selected and saved hash lengths differ.
    HashLengthDiffers(i32, i32),
    /// Parsing the hashes file failed.
    HashesFileParsingFailure(&'err str),
    /// The specified amount of files do not match.
    NFilesDiffer(i32),
}

impl<'err> std::error::Error for Error<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl<'err> fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ArgumentParsingError(err) => write!(f, "CL Argument Parsing Error: {}", err),
            Error::HashLengthDiffers(h1_len, h2_len) => write!(f, "Hash lengths Differ! Hash 1: {} Hash 2: {}", h1_len, h2_len),
            Error::HashesFileParsingFailure(err) => write!(f, "Error while Parsing Files: {}", err),
            Error::NFilesDiffer(num) => write!(f, "Files Differ: {}", num),
        }
    }
}
