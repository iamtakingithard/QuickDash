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

use std::str::FromStr;

use clap::ValueEnum;

/// A hashing algorithm.
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// assert_eq!(
/// 	quickdash::Algorithm::from_str("BLAKE3"),
/// 	Ok(quickdash::Algorithm::BLAKE3)
/// );
/// assert_eq!(
/// 	quickdash::Algorithm::from_str("MD5"),
/// 	Ok(quickdash::Algorithm::MD5)
/// );
/// ```

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Algorithm {
	SHA1,
	SHA2224,
	SHA2256,
	SHA2384,
	SHA2512,
	SHA3224,
	SHA3256,
	SHA3384,
	SHA3512,
	XXH32,
	XXH64,
	XXH3,
	CRC32,
	MD5,
	WhirlPool,
	BLAKE2B,
	BLAKE2S,
	BLAKE3,
}

impl Algorithm {
	/// Length, in bytes, of the algorithm's output hex string
	pub fn hexlen(&self) -> usize {
		match *self {
			Algorithm::CRC32 | Algorithm::XXH32 => 8,
			Algorithm::XXH3 | Algorithm::XXH64 => 16,
			Algorithm::MD5 => 32,
			Algorithm::SHA3256 | Algorithm::SHA2256 | Algorithm::BLAKE2S | Algorithm::BLAKE3 => 64,
			Algorithm::SHA1 => 40,
			Algorithm::SHA2224 | Algorithm::SHA3224 => 56,
			Algorithm::SHA2384 | Algorithm::SHA3384 => 96,
			Algorithm::BLAKE2B | Algorithm::SHA3512 | Algorithm::SHA2512 | Algorithm::WhirlPool => {
				128
			}
		}
	}
}

impl FromStr for Algorithm {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match &s.replace('_', "-").to_lowercase()[..] {
			"sha-1" | "sha1" => Ok(Algorithm::SHA1),
			"sha2224" | "sha-224" | "sha-2-224" => Ok(Algorithm::SHA2224),
			"sha2256" | "sha-256" | "sha-2-256" => Ok(Algorithm::SHA2256),
			"sha2384" | "sha-384" | "sha-2-384" => Ok(Algorithm::SHA2384),
			"sha2512" | "sha-512" | "sha-2-512" => Ok(Algorithm::SHA2512),
			"sha3224" | "sha3-224" | "sha-3-224" => Ok(Algorithm::SHA3224),
			"sha3256" | "sha3-256" | "sha-3-256" => Ok(Algorithm::SHA3256),
			"sha3384" | "sha3-384" | "sha-3-384" => Ok(Algorithm::SHA3384),
			"sha3512" | "sha3-512" | "sha-3-512" => Ok(Algorithm::SHA3512),
			"crc32" => Ok(Algorithm::CRC32),
			"xxhash64" | "xxh64" => Ok(Algorithm::XXH64),
			"xxhash32" | "xxh32" => Ok(Algorithm::XXH32),
			"xxhash3" | "xxh3" => Ok(Algorithm::XXH3),
			"md5" => Ok(Algorithm::MD5),
			"blake2b" => Ok(Algorithm::BLAKE2B),
			"blake2s" => Ok(Algorithm::BLAKE2S),
			"blake3" => Ok(Algorithm::BLAKE3),
			"whirlpool" => Ok(Algorithm::WhirlPool),
			_ => Err(format!("\"{}\" is not a recognised hashing algorithm", s)),
		}
	}
}
