/*
A lib crate for the 'find' program.

This lib crate contains the 'Finder' struct as well as it's public API used
by the application binary crate to create the 'find' program.

The core functionality of the 'Finder' struct is to be able to find files relative
to some given directory. The Finder object utilizes the Builder pattern to allow
users to add filter criteria to narrow down the search results.

 */

use std::collections::VecDeque;
use std::path::PathBuf;
use std::{io, fs};
use io::Error;
use regex::Regex;


pub struct Finder {
    directory: String,
    filters: Vec<Box<dyn Fn(&str) -> bool>>,
}

impl Finder {

    pub fn new(dir: String) -> Finder {
        Finder {
            directory: dir,
            filters: Vec::new(),
        }
    }

    /// Adds the given filter (closure) to this. Does _not_ evaluate it
    /// until a terminal operator is called (lazy). The closure passed to
    /// this function will be used as a filter when searching for files with
    /// the `find()` of `print_find()` function.
    pub fn filter(mut self, predicate: impl Fn(&str) -> bool + 'static) -> Self {
        self.filters.push(Box::new(predicate));
        self
    }

    /// Returns true if file represented by the given &String passes
    /// all of the filters currently in Self.
    fn meets_filter_criteria(&self, file_str: &String) -> bool {
        self.filters.iter().all(|f| f(file_str))
    }

    /// Consumes this Finder (terminal operator). Searches for files starting
    /// from self.root, up to a max depth. Returns the files that
    /// pass all of the filters currently in Self.
    pub fn find(self, depth: u32) -> Result<Vec<String>, Error> {
        // Error check for the root dir to exits before starting.
        let root = PathBuf::from(&self.directory);
        if !root.exists() {
            return Err(Error::new(
                io::ErrorKind::NotFound,
                format!("Root directory {} does not exists.", self.directory)));
        }
        let mut result = Vec::new();
        let mut queue: VecDeque<PathBuf> = VecDeque::new();
        queue.push_back(root);
        let mut curr_depth = 0;

        // Use BFS to search files one depth layer at a time. For a given item found,
        // If it's a dir, add it's children to the queue as long as max depth not reached.
        // If it's a file, add it to result if it passes our filters.
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let path = queue.pop_front().unwrap();
                if path.is_dir() && curr_depth <= depth {
                    for entry in fs::read_dir(path)? {
                        let child = entry?.path();
                        queue.push_back(child);
                    }
                } else if path.is_file() {
                    let path_string = String::from(path.into_os_string().into_string().unwrap());
                    if self.meets_filter_criteria(&path_string) {
                        result.push(path_string);
                    }
                }
            }
            curr_depth += 1;
        }
        Ok(result)
    }

    /// Consumes this Finder (terminal operator). Searches for files starting
    /// from self.root, up to a max depth. Prints the files that
    /// pass all of the filters currently in Self.
    pub fn print_find(self, depth: u32) -> Result<(), Error> {
        // Error check for the root dir to exits before starting.
        let root = PathBuf::from(&self.directory);
        if !root.exists() {
            return Err(Error::new(
                io::ErrorKind::NotFound,
                format!("Root directory {} does not exists.", self.directory)));
        }
        let mut queue: VecDeque<PathBuf> = VecDeque::new();
        queue.push_back(root);
        let mut curr_depth = 0;

        // Use BFS to search files one depth layer at a time. For a given item found,
        // If it's a dir, add it's children to the queue as long as max depth not reached.
        // If it's a file, add it to result if it passes our filters.
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let path = queue.pop_front().unwrap();
                if path.is_dir() && curr_depth <= depth {
                    for entry in fs::read_dir(path)? {
                        let entry = entry?;
                        let child = entry.path();
                        queue.push_back(child);
                    }
                } else if path.is_file() {
                    let path_string = String::from(path.into_os_string().into_string().unwrap());
                    if self.meets_filter_criteria(&path_string) {
                        println!("matching file: {}", path_string);
                    }
                }
            }
            curr_depth += 1;
        }
        Ok(())
    }

    /// Adds a filter to this `Finder` that retains files with a size less
    /// than or equal to the given size `bytes`.
    pub fn size_less_than_or_eq(self, bytes: u32) -> Finder {
        self.filter(move |s| {
            match fs::metadata(s) {
                Ok(meta) => meta.len() <= bytes as u64,
                Err(_) => false
            }
        })
    }

    /// Adds a filter to this `Finder` that retains files with a size greater
    /// than or equal to the given size `bytes`.
    pub fn size_greater_than_or_eq(self, bytes: u32) -> Finder {
        self.filter(move |s| {
            match fs::metadata(s) {
                Ok(meta) => meta.len() >= bytes as u64,
                Err(_) => false
            }
        })
    }

    /// Adds a filter to this `Finder` that retains files with the given extension `ext`
    /// (case sensitive).
    ///
    /// This filter is lazy and isn't actually applied until this `Finder` is consumed.
    pub fn has_extension(self, ext: String) -> Self {
        self.filter(move |s| s.ends_with(&ext))
    }

    /// Adds a filter to this `Finder` that retains files with the given extension `ext`
    /// (case insensitive). This filter is slightly slower than `has_extension()` since
    /// it has to cast both the file name and the extension to lowercase for comparison.
    ///
    /// This filter is lazy and isn't actually applied until this `Finder` is consumed.
    pub fn has_extension_case_insensitive(self, ext: String) -> Self {
        self.filter(move |s| s.to_lowercase().ends_with(&ext.to_lowercase()))
    }

    /// Adds a filter to this `Finder` that retains files for which the given regex pattern
    /// is found in the file name. Does not need to match the entire file name.
    pub fn matches_regex(self, pattern: &str) -> Finder {
        let re = Regex::new(pattern).unwrap();
        self.filter(move |s| {
            if let Some(name) = PathBuf::from(s).file_name() {
                if let Some(name) = name.to_str() {
                    return re.is_match(name);
                }
            }
            false
        })
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_src_files() {
        let files = Finder::new("src".to_string())
            .find(0)
            .unwrap();
        println!("files = {:#?}", files);
        assert_eq!(true, files.contains(&"src/lib.rs".to_string()));
    }

    #[test]
    fn non_existing_root_dir() {
        let result = Finder::new("non_existing_dir/".to_string())
            .find(0);
        assert!(result.is_err());
    }

    #[test]
    fn print_find() {
        let result = Finder::new("src/".to_string())
            .print_find(1);
        assert!(result.is_ok());
    }

    #[test]
    fn has_extension_lock() {
        let result = Finder::new("./".to_string())
            .has_extension(String::from(".lock"))
            .find(0)
            .unwrap();
        assert_eq!(1, result.len());
    }

    #[test]
    fn files_gt_10_b() {
        let result = Finder::new("src/".to_string())
            .has_extension(String::from(".rs"))
            .size_greater_than_or_eq(10)
            .find(0)
            .unwrap();
        assert_eq!(2, result.len(), "There should be 3 source files with size >= 10 B.")
    }

    #[test]
    fn files_gt_1_mb() {
        let result = Finder::new("src/".to_string())
            .has_extension(String::from(".rs"))
            .size_greater_than_or_eq(1_000_000)
            .find(0)
            .unwrap();
        assert_eq!(0, result.len(), "There should be 0 source files with size >= 1 MB.")
    }

    #[test]
    fn files_lt_10_b() {
        let result = Finder::new("src/".to_string())
            .has_extension(String::from(".rs"))
            .size_less_than_or_eq(10)
            .find(0)
            .unwrap();
        assert_eq!(0, result.len(), "There should be 0 source files with size <= 10 B.")
    }

    #[test]
    fn files_lt_1_mb() {
        let result = Finder::new("src/".to_string())
            .has_extension(String::from(".rs"))
            .size_less_than_or_eq(1_000_000)
            .find(0)
            .unwrap();
        assert_eq!(2, result.len(), "There should be 3 source files with size <= 1 MB.")
    }

    #[test]
    fn custom_filter_for_letter_n() {
        let finder = Finder::new("src/".to_string());
        let result = finder
            .filter(|file_name| file_name.contains("n"))
            .find(3)
            .unwrap();
        assert_eq!(1, result.len(), "There should be 2 src/ files with 'n' in name.")
    }

    #[test]
    fn has_extension_rs_case_sensitive() {
        let result = Finder::new("./".to_string())
            .has_extension(String::from(".rs"))
            .find(1)
            .unwrap();
        assert_eq!(2, result.len(), "There should be 3 source files with '.rs' extension.");
        let result = Finder::new("./".to_string())
            .has_extension(String::from(".RS"))
            .find(1)
            .unwrap();
        assert_eq!(0, result.len(), "There should be 0 source files with '.RS' extension.");
    }

    #[test]
    fn has_extension_rs_case_insensitive() {
        let result = Finder::new("./".to_string())
            .has_extension_case_insensitive(String::from(".rs"))
            .find(1)
            .unwrap();
        assert_eq!(2, result.len(), "There should be 3 source files with '.rs' extension.");
        let result = Finder::new("./".to_string())
            .has_extension_case_insensitive(String::from(".RS"))
            .find(1)
            .unwrap();
        assert_eq!(2, result.len(), "There should be 3 source files matching '.RS' extension.");
    }

    #[test]
    fn matches_regex_test() {
        let result = Finder::new("./".to_string())
            .matches_regex(r".*\.rs")
            .find(1)
            .unwrap();
        assert_eq!(2, result.len());
        let result = Finder::new("./".to_string())
            .matches_regex(r"^l.*\.rs")
            .find(1)
            .unwrap();
        assert_eq!(1, result.len());
    }

}
