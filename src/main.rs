/*
Main source code for the 'find' program.

Requirements

    CLI:
    -d --depth - recursion depth (sub-folder depth).
    -p --pattern - regex pattern to match against file names.
    -ext --extension - file extension.
    -sz-l --size-less - file size must be <= this.
    -sz-g --size-greater - file size must be >= this.

 */

use find::finder::Finder;

fn main() {
    let finder = Finder::new(String::from("mydir/"));
    let files = finder
        .filter(|s| s.ends_with(".xml"))
        .filter(|s| s.contains("3"))
        .find();
    println!("finder:");
    println!("  files: {:?}", files);
}


