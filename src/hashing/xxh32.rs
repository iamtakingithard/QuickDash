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

hash_func!(
	xxhash_rust::xxh32::Xxh32::new(1234),
	|xxh32: &mut xxhash_rust::xxh32::Xxh32, buffer: &[u8]| xxh32.update(buffer),
	|xxh32: xxhash_rust::xxh32::Xxh32| format!("{:08X}", (&xxh32.digest()))
);
