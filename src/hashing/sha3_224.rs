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

use sha3::{Digest, Sha3_224};

use crate::hash_string;

hash_func!(
	Sha3_224::new(),
	|sha3224: &mut Sha3_224, buffer: &[u8]| sha3224.update(buffer),
	|sha3224: Sha3_224| hash_string(&sha3224.finalize())
);
