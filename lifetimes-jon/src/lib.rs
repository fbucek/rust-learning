#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> { // Valid as long as references heystack both exists 
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}


impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder { // ref mut ( not moving out of self. ) Option<&'a str> -> &mut &'a str
        // if let Some(remainder) = &mut self.remainder { // ekvivalent without ref mut
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                // remainder -> delmiter 
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)

            } else {
                self.remainder.take() // return Some(T) leaving None or 
                // 
            }
        } else {
            None
        }
        // } else if let Some(remainder) = self.remainder.take() {
        //     // TODO: bug
        //     Some(remainder)
        // } else {
        //     None
        // }
    }
}


#[test]
fn it_works() {
    let haystack = "a b c d e";
    // Iterator check
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
    // Vector check
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}


#[test]
fn tail() {
    let haystack = "a b c d ";
    // Vector check
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
