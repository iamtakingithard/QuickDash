use quick_dash::Algorithm;
use std::str::FromStr;

#[test]
fn from_str() {
    for a in &[("sha1", Algorithm::SHA1),
               ("sha-224", Algorithm::SHA2224),
               ("sha-256", Algorithm::SHA2256),
               ("sha-384", Algorithm::SHA2384),
               ("sha-512", Algorithm::SHA2512),
               ("sha3-224", Algorithm::SHA3224),
               ("sha3-256", Algorithm::SHA3256),
               ("sha3-384", Algorithm::SHA3384),
               ("sha3-512", Algorithm::SHA3512),
               ("blake2b", Algorithm::BLAKE2B),
               ("blake2s", Algorithm::BLAKE2S),
               ("blake3", Algorithm::BLAKE3),
               ("xxh3", Algorithm::XXH3),
               ("xxh64", Algorithm::XXH64),
               ("xxh32", Algorithm::XXH32),
               ("crc32", Algorithm::CRC32),
               ("md5", Algorithm::MD5),
               ("whirlpool", Algorithm::WhirlPool)] {
        assert_eq!(Algorithm::from_str(a.0).unwrap(), a.1);
    }
}
