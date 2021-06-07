/*
Main source code for the 'find' program.

Requirements
   A module that can be used by other rust programs and provides API methods
   for finding files. The API supports
   - finding files relative to some given directory.
   - The users can provide a Predicate to filter the files with (or more than one predicate).
   - The user can also specify a max depth for subdirectories ot check.

 */
#![allow(dead_code)]

use std::collections::VecDeque;
use std::path::{PathBuf};
use std::{io, fs};
use io::Error;
use std::fs::Metadata;


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

    // Adds the given filter to this, does not evaluate it until terminal operator is called (lazy).
    pub fn filter(mut self, predicate: impl Fn(&str) -> bool + 'static) -> Finder {
        self.filters.push(Box::new(predicate));
        self
    }

    // Terminal operator. Triggers the find to begin and applies all filters previously set.
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
                        let entry = entry?;
                        let child = entry.path();
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

    fn meets_filter_criteria(&self, file_str: &String) -> bool {
        self.filters.iter().all(|f| (f)(file_str))
    }

    // Terminal operator. Prints each file that it finds as it finds them.
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

    pub fn size_less_than_or_eq(mut self, bytes: u32) -> Finder {
        self.filter(move |s| {
            match fs::metadata(s) {
                Ok(meta) => meta.len() <= bytes as u64,
                Err(_) => false
            }
        })
    }

    pub fn size_greater_than_or_eq(mut self, bytes: u32) -> Finder {
        self.filter(move |s| {
            match fs::metadata(s) {
                Ok(meta) => meta.len() >= bytes as u64,
                Err(_) => false
            }
        })
    }

    // Filters the files such to retain files with the given extension. Convenience function.
    pub fn has_extension(mut self, ext: &'static str) -> Finder {
        self.filter(move |s| s.ends_with(ext))
    }

    pub fn matches_regex(mut self, pattern: &str) -> Finder {
        // TODO
        self
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn my_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn finds_src_files() {
        let files = Finder::new("src".to_string())
            .find(0)
            .unwrap();
        println!("files = {:#?}", files);
        assert_eq!(true, files.contains(&"src/finder.rs".to_string()));
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
            .has_extension(".lock")
            .find(0)
            .unwrap();
        assert_eq!(1, result.len());
    }

    #[test]
    fn files_gt_10_B() {
        let result = Finder::new("src/".to_string())
            .has_extension(".rs")
            .size_greater_than_or_eq(10)
            .find(0)
            .unwrap();
        assert_eq!(3, result.len(), "There should be 3 source files with size >= 10 B.")
    }

    #[test]
    fn files_gt_1_MB() {
        let result = Finder::new("src/".to_string())
            .has_extension(".rs")
            .size_greater_than_or_eq(1_000_000)
            .find(0)
            .unwrap();
        assert_eq!(0, result.len(), "There should be 0 source files with size >= 1 MB.")
    }

    #[test]
    fn files_lt_10_B() {
        let result = Finder::new("src/".to_string())
            .has_extension(".rs")
            .size_less_than_or_eq(10)
            .find(0)
            .unwrap();
        assert_eq!(0, result.len(), "There should be 0 source files with size <= 10 B.")
    }

    #[test]
    fn files_lt_1_MB() {
        let result = Finder::new("src/".to_string())
            .has_extension(".rs")
            .size_less_than_or_eq(1_000_000)
            .find(0)
            .unwrap();
        assert_eq!(3, result.len(), "There should be 3 source files with size <= 1 MB.")
    }

}
