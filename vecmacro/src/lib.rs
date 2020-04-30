#[allow(unused_macros)]
macro_rules! avec {
    () => { Vec::new() };
    // $($element:expr),+   -- for $element:expr divided by ',' + ( at least one )
    // $(,)?                -- ',' can be there on  --> trailing ,
    ($($element:expr),+ $(,)?) => {{
        let mut vs = Vec::new();
        // Smiliar syntax 
        $( // for array 
            vs.push($element);
        )+ // do repetition
        vs
    }};
}

#[test]
fn empty_vec() {
    let x: Vec<i32> = avec![];
    assert!(x.is_empty());
    let x: Vec<i32> = avec!();
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<i32> = avec![42];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
    let x: Vec<i32> = avec!(42);
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<i32> = avec![42, 12];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 12);
    let x: Vec<i32> = avec!(42, 12);
    assert_eq!(x.len(), 2);
}

#[test]
fn trailing() {
    let x: Vec<&'static str> = avec![
        "first string", 
        "second string", 
        ];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], "first string");
    assert_eq!(x[1], "second string");
}
