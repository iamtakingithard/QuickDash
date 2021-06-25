```
QuickDash 0.5.0
A modern alternative to QuickSFV using Rust. Made with <3 by Cerda.

USAGE:
    quickdash [FLAGS] [OPTIONS] [--] [DIRECTORY]

FLAGS:
    -c, --create                Make hashes
        --follow-symlinks       Recurse down symlinks. Default: yes
        --force                 Override output file
    -h, --help                  Prints help information
        --no-follow-symlinks    Don't recurse down symlinks
    -r, --recursive             Infinite recursion depth.
    -V, --version               Prints version information
    -v, --verify                Verify hashes (default)

OPTIONS:
    -a, --algorithm <algorithm>
            Hashing algorithm to use.
            Supported algorithms: SHA{1,2-,3-{224,256,384,512}, CRC32, MD5, BLAKE{2B,2S,3}, XXHASH{3,64} [default:
            BLAKE3]
    -d, --depth <depth>            Max recursion depth. `-1` for infinite.'. Default: don't recurs
    -f, --file <file>              File with hashes to be read/created
    -i, --ignore <file>...         Ignore specified file(s)
    -j, --jobs <jobs>              # of threads used for hashing. No/empty value: # of CPU threads. value = 0: maximum
                                   of u8 (255)

ARGS:
    <DIRECTORY>    Directory to hash/verify [default: .]
```
