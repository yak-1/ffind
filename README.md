# rustfind
A file find utility modeled after the unix `find` written in Rust.

## Why
This project is used as an intro to Rust exersize. 
Implementing `find` is great because it allows you to utilize many concepts that you may be already familiar with from other languages in a quick
and concise project to get a feel for how to do those things in Rust. In specific, this project calls for the use of recursion, tree traversal, CLI, 
file IO, structs, error handling, closures, lifetimes, regex,
and testing.

## Usage

```
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

```

## Example
```
% rustfind --pattern 'ma.n{1}' --extension '.rs' ./rustlings
matching file: rustlings/src/main.rs
```
