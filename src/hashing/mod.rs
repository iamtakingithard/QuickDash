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

macro_rules! hash_func {
	($ctx:expr, $update:expr, $convert:expr) => {
		use std::io::Read;

		pub fn hash<R: Read>(reader: &mut R) -> String {
			let mut buffer = vec![0; 4096];

			let mut ctx = $ctx;
			loop {
				let read = reader.read(&mut buffer[..]).unwrap();

				if read == 0 {
					break;
				}

				$update(&mut ctx, &buffer[..read]);
			}

			$convert(ctx)
		}
	};
}

macro_rules! hash_func_write {
	($ctx:expr, $convert:expr) => {
		use std::io::{self, Read};

		pub fn hash<R: Read>(reader: &mut R) -> String {
			let mut ctx = $ctx;
			io::copy(reader, &mut ctx).unwrap();
			$convert(ctx)
		}
	};
}

use std::{fmt::Write, fs::File, io::Read, path::Path};

use super::Algorithm;

mod blake2b;
mod blake2s;
mod blake3;
mod crc32;
mod md5;
mod sha1;
mod sha2_224;
mod sha2_256;
mod sha2_384;
mod sha2_512;
mod sha3_224;
mod sha3_256;
mod sha3_384;
mod sha3_512;
mod whirlpool;
mod xxh3;
mod xxh32;
mod xxh64;

/// Hash the specified file using the specified hashing algorithm.
pub fn hash_file(algo: Algorithm, path: &Path) -> String {
	hash_reader(algo, &mut File::open(path).unwrap())
}

/// Hash the specified byte stream using the specified hashing algorithm.
pub fn hash_reader<R: Read>(algo: Algorithm, data: &mut R) -> String {
	match algo {
		Algorithm::CRC32 => crc32::hash(data),
		Algorithm::SHA1 => sha1::hash(data),
		Algorithm::SHA2224 => sha2_224::hash(data),
		Algorithm::SHA2256 => sha2_256::hash(data),
		Algorithm::SHA2384 => sha2_384::hash(data),
		Algorithm::SHA2512 => sha2_512::hash(data),
		Algorithm::SHA3224 => sha3_224::hash(data),
		Algorithm::SHA3256 => sha3_256::hash(data),
		Algorithm::SHA3384 => sha3_384::hash(data),
		Algorithm::SHA3512 => sha3_512::hash(data),
		Algorithm::MD5 => md5::hash(data),
		Algorithm::XXH64 => xxh64::hash(data),
		Algorithm::XXH32 => xxh32::hash(data),
		Algorithm::XXH3 => xxh3::hash(data),
		Algorithm::BLAKE2B => blake2b::hash(data),
		Algorithm::BLAKE2S => blake2s::hash(data),
		Algorithm::BLAKE3 => blake3::hash(data),
		Algorithm::WhirlPool => whirlpool::hash(data),
	}
}

/// Create a hash string out of its raw bytes.
///
/// # Examples
///
/// ```
/// assert_eq!(
/// 	quickdash::hash_string(&[0x99, 0xAA, 0xBB, 0xCC]),
/// 	"99AABBCC".to_string()
/// );
/// assert_eq!(quickdash::hash_string(&[0x09, 0x0A]), "090A".to_string());
/// ```
pub fn hash_string(bytes: &[u8]) -> String {
	let mut result = String::with_capacity(bytes.len() * 2);
	for b in bytes {
		write!(result, "{:02X}", b).unwrap();
	}
	result
}
