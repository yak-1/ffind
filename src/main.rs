/*

find

Jack D. <jrd666@protonmail.com>
finds files

USAGE:
    find [OPTIONS] <PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --depth <DEPTH>                Configures the max depth this recursive search will explore [default: 99999]
    -e, --extension <EXT>              Looks for files that have this file extension
    -p, --pattern <REGEX>              Looks for files that contain this REGEX
    -g, --size-greater-than <BYTES>    filters files where file size is not >= BYTES
    -l, --size-less-than <BYTES>       filters files where file size is not <= BYTES

ARGS:
    <PATH>    Initial location to begin the search

 */

use find::Finder;
use clap::{Arg, App};
use std::path::PathBuf;

struct Config {
    root: String,
    depth: u32,
    file_extension: Option<String>,
    pattern: Option<String>,
    size_greater_than: Option<u32>,
    size_less_than: Option<u32>,
}


impl Config {

    fn new() -> Config {
        let matches = App::new("find")
            .version("0.1.0")
            .author("Jack D. <jrd666@protonmail.com>")
            .about("finds files")
            .arg(Arg::with_name("PATH")
                .help("Initial location to begin the search")
                .required(true)
                .index(1))
            .arg(Arg::with_name("size-less-than")
                .short("l")
                .long("size-less-than")
                .takes_value(true)
                .value_name("BYTES")
                .multiple(false)
                .help("filters files where file size is not <= BYTES"))
            .arg(Arg::with_name("size-greater-than")
                .short("g")
                .long("size-greater-than")
                .takes_value(true)
                .value_name("BYTES")
                .multiple(false)
                .help("filters files where file size is not >= BYTES"))
            .arg(Arg::with_name("depth")
                .short("d")
                .long("depth")
                .takes_value(true)
                .value_name("DEPTH")
                .default_value("99999")
                .multiple(false)
                .help("Configures the max depth this recursive search will explore"))
            .arg(Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .takes_value(true)
                .value_name("REGEX")
                .multiple(false)
                .help("Looks for files that contain this REGEX"))
            .arg(Arg::with_name("extension")
                .short("e")
                .long("extension")
                .takes_value(true)
                .value_name("EXT")
                .multiple(false)
                .help("Looks for files that have this file extension"))
            .get_matches();

        // Extract the search root. Check to make sure it exists.
        let root = matches.value_of("PATH").unwrap().to_string();
        if !PathBuf::from(&root).exists() {
            eprintln!("ERROR: Invalid argument for PATH: <{}>. Make sure search path exists.", root);
            std::process::exit(1);
        }

        // Extract the depth argument and check for errors.
        let depth: u32 = match matches.value_of("depth").unwrap().parse() {
            Ok(depth) => depth,
            Err(e) => {
                eprintln!("ERROR: Invalid argument --depth: {}.", e);
                std::process::exit(1);
            }
        };

        let file_extension = if let Some(s) = matches.value_of("extension") {
            Some(s.to_string())
        } else {
            None
        };

        let pattern = if let Some(s) = matches.value_of("pattern") {
            Some(s.to_string())
        } else {
            None
        };

        let size_less_than: Option<u32> = match matches.value_of("size-less-than") {
            Some(bytes) => Some(bytes.parse().unwrap_or_else(|e| {
                eprintln!("ERROR: Invalid argument --size-less-than: {}.", e);
                std::process::exit(1);
            })),
            None => None,
        };

        let size_greater_than: Option<u32> = match matches.value_of("size-greater-than") {
            Some(bytes) => Some(bytes.parse().unwrap_or_else(|e| {
                eprintln!("ERROR: Invalid argument --size-greater-than: {}.", e);
                std::process::exit(1);
            })),
            None => None,
        };

        // Return the Config struct with the fields now that error checking is complete.
        Config {
            root,
            depth,
            file_extension,
            pattern,
            size_greater_than,
            size_less_than
        }
    }
}

fn main() {
    let config = Config::new();
    let mut finder = Finder::new(config.root);

    if let Some(size) = config.size_less_than {
        finder = finder.size_less_than_or_eq(size);
    };

    if let Some(size) = config.size_greater_than {
        finder = finder.size_greater_than_or_eq(size);
    };

    if let Some(ext) = config.file_extension {
        finder = finder.has_extension_case_insensitive(ext)
    };

    if let Some(pattern) = config.pattern {
        finder = finder.matches_regex(&pattern);
    };

    // Consume the finder and print the results.
    let _ = finder.print_find(config.depth);

}


