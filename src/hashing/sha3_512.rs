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

use sha3::{Digest, Sha3_512};

use crate::hash_string;

hash_func!(
	Sha3_512::new(),
	|sha3512: &mut Sha3_512, buffer: &[u8]| sha3512.update(buffer),
	|sha3512: Sha3_512| hash_string(&sha3512.finalize())
);
