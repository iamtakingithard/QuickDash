```
QuickDash 0.6.0
A modern alternative to QuickSFV using Rust. Made with <3 by Cerda.

USAGE:
    quickdash [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -a, --algorithm <ALGORITHM>
            Hashing algorithm to use
            
            [default: blake3]
            [possible values: sha1, sha2224, sha2256, sha2384, sha2512, sha3224, sha3256, sha3384,
            sha3512, xxh32, xxh64, xxh3, crc32, md5, whirl-pool, blake2b, blake2s, blake3]

    -d, --depth <DEPTH>
            Max recursion depth. Infinite if None. Default: `0`

        --follow-symlinks
            Whether to recurse down symlinks. Default: `true`

    -h, --help
            Print help information

    -i, --ignored-files <IGNORED_FILES>
            Files/directories to ignore. Default: none

    -j, --jobs <JOBS>
            # of threads used for hashing
            
            [default: 0]

    -V, --version
            Print version information

SUBCOMMANDS:
    create
            
    help
            Print this message or the help of the given subcommand(s)
    verify
```

```
quickdash-create 

USAGE:
    quickdash create [OPTIONS] [PATH]

ARGS:
    <PATH>    Directory to hash. Default: current directory [default: .]

OPTIONS:
    -f, --force          
        --file <FILE>    Output filename. Default: `directory_name.hash"`
    -h, --help           Print help information
```

```
quickdash-verify 

USAGE:
    quickdash verify [OPTIONS] [PATH]

ARGS:
    <PATH>    Directory to verify. Default: current directory [default: .]

OPTIONS:
        --file <FILE>    Input filename. Default: `directory_name.hash"`
    -h, --help           Print help information
```