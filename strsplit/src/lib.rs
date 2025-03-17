
pub struct StrSplit<'a>{
    remainder: &'a str, // part of the string that we havent looked at
    delimiter: &'a str //splitting by. 
}

impl StrSplit<'_> {
    pub fn new(haystack: &str, delimiter: &str) -> Self {
        Self {
            remainder: haystack,
            delimiter
        }
    }
}
// let x: StrSplit;
// for part in x {

// }
//implement iterator for a strsplit

impl<'a> Iterator for StrSplit<'a>{
   type Item = &str; 
    // the next method needs a mutable reference to the self
    // It returns an Optional Item. 
    fn next(&mut self)-> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[next_delim + self.delimiter.len()..];
            Some(until_delimiter)
        } else {
            self.remainder
        }
    }
}

// Takes some string and returns 
// a vector of string splits. 
#[test]
fn test_strsplit() {
    let haystack = "a b c d e f";
    for part in StrSplit::new(haystack, " ") {
        // append all the parts with the delimiter and join them
        // so that it quals to haysatck

    }
    assert_eq!(haysatck," "); 
}
