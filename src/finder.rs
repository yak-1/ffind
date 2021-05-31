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


pub struct Finder {
    directory: String,
    filters: Vec<fn(&String) -> bool>,
}

impl Finder {

    pub fn new(dir: String) -> Box<Finder> {
        Box::new(Finder {
            directory: dir,
            filters: Vec::new(),
        })
    }

    // Applies the given filter to this, does not evaluate it until terminal operator is called.
    pub fn filter(mut self, predicate: fn(&String) -> bool) -> Finder {
        self.filters.push(predicate);
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
        self.filters.iter().all(|f| f(file_str))
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

}
