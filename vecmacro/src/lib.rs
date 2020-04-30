#[allow(unused_macros)]
macro_rules! avec {
    () => { Vec::new() };
    ($element:expr) => {{
        let mut vs = Vec::new();
        vs.push($element);
        vs
    }};
    ($e1:expr, $e2:expr) => {{
        let mut vs = Vec::new();
        vs.push($e1);
        vs.push($e2);
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
