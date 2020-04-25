#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> { // Valid as long as references heystack both exists 
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}


impl<'haystack> Iterator for StrSplit<'haystack, '_> {
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?; // this modyfies 
        // if let Some(ref mut remainder) = self.remainder { // ref mut ( not moving out of self. ) Option<&'a str> -> &mut &'a str
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
    }
}

fn until_char(s: &str, c: char) -> &str { 
    let delim = format!("{}", c);
    StrSplit::new(s, &delim)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
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
