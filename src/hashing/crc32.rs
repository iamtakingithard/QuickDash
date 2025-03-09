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

hash_func!(
	crc32fast::Hasher::new(),
	|crc: &mut crc32fast::Hasher, buffer: &[u8]| crc.update(buffer),
	|crc: crc32fast::Hasher| format!("{:08X}", (&crc.finalize()))
);
