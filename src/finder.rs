/*
Main source code for the 'find' program.

Requirements
   A module that can be used by other rust programs and provides API methods
   for finding files. The API supports
   - finding files relative to some given directory.
   - The users can provide a Predicate to filter the files with (or more than one predicate).
   - The user can also specify a max depth for subdirectories ot check.

 */

pub struct Finder {
    directory: String,
    files: Vec<String>,
    filters: Vec<fn(&String) -> bool>,
}

impl Finder {

    pub fn new(dir: String) -> Box<Finder> {
        Box::new(Finder {
            directory: dir,
            files: Vec::new(),
            filters: Vec::new(),
        })
    }

    // Applies the given filter to this, does not evaluate it until terminal operator is called.
    pub fn filter(mut self, predicate: fn(&String) -> bool) -> Finder {
        self.filters.push(predicate);
        self
    }

    // Terminal operator. Exhausts self.
    pub fn find(mut self, depth: u32) -> Vec<String> {
        for predicate in self.filters {
            self.files = self.files.into_iter()
                .filter(|s| predicate(s))
                .collect();
        }
        self.files
    }

    // Terminal operator. Prints each file that it finds as it finds them.
    pub fn print_find(mut self, depth: u32) {
        // TODO
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn my_test() {
        assert_eq!(true, true);
    }
}
