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

use crate::Algorithm;
use clap::{self, crate_version, App, AppSettings, Arg};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// Directory to hash/verify. Default: current directory
    pub dir: PathBuf,
    /// Hashing algorithm to use. Default: `"BLAKE3"`
    pub algorithm: Algorithm,
    /// Whether to verify or create hashes. Default: yes
    pub verify: bool,
    /// Max recursion depth. Infinite if None. Default: `0`
    pub depth: Option<usize>,
    /// In-/Output filename. Default: `"directory_name.hash"`
    pub file: (String, PathBuf),
    /// Whether to recurse down symlinks. Default: `true`
    pub follow_symlinks: bool,
    /// Files/directories to ignore. Default: none
    pub ignored_files: BTreeSet<String>,
    /// # of threads used for hashing.
    pub jobs: usize,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = App::new("QuickDash")
            .setting(AppSettings::ColoredHelp)
            .about("A modern alternative to QuickSFV using Rust. Made with <3 by Cerda.")
            .version(crate_version!())
            .args(&[
                Arg::from_usage("[DIRECTORY] 'Directory to hash/verify'")
                    .default_value(".")
                    .validator(Options::directory_validator),
                Arg::from_usage(
                    "--algorithm=[algorithm] -a 'Hashing algorithm to use. {n}\
                    Supported algorithms: SHA{1,2-,3-{224,256,384,512}, CRC32, MD5, BLAKE{2B,2S,3}, XXHASH{3,64}'",
                )
                .next_line_help(true)
                .default_value("BLAKE3")
                .validator(Options::algorithm_validator),
                Arg::from_usage("--create -c 'Make hashes'").overrides_with("verify"),
                Arg::from_usage("--verify -v 'Verify hashes (default)'").overrides_with("create"),
                Arg::from_usage("--depth=[depth] -d 'Max recursion depth. `-1` for infinite.'. Default: don't recurse")
                    .validator(Options::depth_validator)
                    .overrides_with("recursive"),
                Arg::from_usage("--recursive -r 'Infinite recursion depth.'").overrides_with("depth"),
                Arg::from_usage("--file=[file] -f 'File with hashes to be read/created'").validator(Options::file_validator),
                Arg::from_usage("--force 'Override output file'"),
                Arg::from_usage("--follow-symlinks 'Recurse down symlinks. Default: yes'").overrides_with("no-follow-symlinks"),
                Arg::from_usage("--no-follow-symlinks 'Don\'t recurse down symlinks'").overrides_with("follow-symlinks"),
                Arg::from_usage("-i --ignore [file]... 'Ignore specified file(s)'"),
                Arg::from_usage("-j --jobs=[jobs] '# of threads used for hashing. No/empty value: # of CPU threads. value = 0: maximum of u8 (255)'")
                    .empty_values(true)
                    .allow_hyphen_values(false)
                    .validator(Options::jobs_validator),
            ])
            .get_matches();

        let dir = fs::canonicalize(matches.value_of("DIRECTORY").unwrap()).unwrap();
        let verify = !matches.is_present("create");
        let file = Options::file_process(matches.value_of("file"), &dir);

        if file.1.exists() && !verify && !matches.is_present("force") {
            clap::Error {
                message: "The output file exists and was not overridden to prevent data loss.\n\
                              Pass the --force option to suppress this error."
                    .to_string(),
                kind: clap::ErrorKind::MissingRequiredArgument,
                info: None,
            }
            .exit();
        } else if !file.1.exists() && verify {
            clap::Error {
                message: format!(
                    "Unable to detect hash list file \"{}\".\n\
                    Did you mean to create it with -c?",
                    file.0
                ),
                kind: clap::ErrorKind::InvalidValue,
                info: None,
            }
            .exit();
        }

        Options {
            dir,
            algorithm: Algorithm::from_str(matches.value_of("algorithm").unwrap()).unwrap(),
            verify,
            depth: if matches.is_present("recursive") {
                None
            } else {
                let i = matches
                    .value_of("depth")
                    .map(|s| s.parse::<isize>().unwrap())
                    .unwrap_or(0);
                if i < 0 {
                    None
                } else {
                    Some(i as usize)
                }
            },
            file,
            follow_symlinks: !matches.is_present("no-follow-symlinks"),
            ignored_files: matches
                .values_of("ignore")
                .map(|v| v.map(String::from).collect())
                .unwrap_or_default(),
            jobs: match matches.value_of("jobs") {
                None => num_cpus::get(),
                Some(s) => match usize::from_str(s) {
                    Ok(num) => match num {
                        0 => u8::MAX as usize,
                        i if i > 0 => i,

                        _ => panic!("Number of jobs cannot be negative!"),
                    },
                    Err(num) => panic!("{}", num),
                },
            },
        }
    }

    fn algorithm_validator(s: String) -> Result<(), String> {
        Algorithm::from_str(&s).map(|_| ())
    }

    fn directory_validator(s: String) -> Result<(), String> {
        fs::canonicalize(s)
            .map_err(|e| format!("directory: {}", e.to_string()))
            .and_then(|p| {
                if p.is_file() {
                    Err("DIRECTORY cannot be a file.".to_string())
                } else {
                    Ok(())
                }
            })
    }

    fn depth_validator(s: String) -> Result<(), String> {
        s.parse::<isize>().map(|_| ()).map_err(|e| e.to_string())
    }

    fn file_validator(s: String) -> Result<(), String> {
        let mut buf = PathBuf::from(s);
        if buf.exists() && buf.is_dir() {
            Err("file exists and is a directory".to_string())
        } else {
            buf.pop();

            // Handle pathless filename
            if buf.as_os_str().is_empty() {
                Ok(())
            } else {
                buf.canonicalize()
                    .map(|_| ())
                    .map_err(|e| format!("file: {}", e.to_string()))
            }
        }
    }

    fn jobs_validator(s: String) -> Result<(), String> {
        if s.is_empty() {
            return Ok(());
        }
        usize::from_str(&s)
            .map_err(|e| format!("jobs: {}", e))
            .and_then(|_| Err("Not a valid amount of jobs!".to_string()))
    }

    fn file_process(file: Option<&str>, dir: &Path) -> (String, PathBuf) {
        match file {
            Some(file) => {
                let mut file = PathBuf::from(file);
                let file_name = file.file_name().unwrap().to_os_string();

                file.pop();
                // Handle pathless filename
                if file.as_os_str().is_empty() {
                    file.push(".");
                }

                (
                    file_name.to_str().unwrap().to_string(),
                    file.canonicalize()
                        .map(|mut p| {
                            p.push(file_name);
                            p
                        })
                        .unwrap(),
                )
            }
            None => {
                let mut file = dir.to_path_buf();
                match dir.file_name() {
                    Some(fname) => file.push(fname),
                    None => file.push(Options::root_fname(dir)),
                }
                file.set_extension("hash");

                (
                    file.file_name().unwrap().to_str().unwrap().to_string(),
                    file,
                )
            }
        }
    }

    #[cfg(windows)]
    fn root_fname(dir: &Path) -> String {
        let dir = dir.as_os_str().to_str().unwrap().to_string();
        dir[dir.len() - 3..dir.len() - 2].to_string()
    }

    #[cfg(not(windows))]
    fn root_fname(_: &Path) -> String {
        "root".to_string()
    }
}
