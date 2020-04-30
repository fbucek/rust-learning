#![allow(unused_macros)] // #! -> for all # only for next

#[cfg(target_os = "linux")]
use std::i32;

///```compile_fail
/// let x: Vec<u32> = vecmacro::avec![42, "foo"];
///```
#[allow(dead_code)]
struct CompileFailTest;

macro_rules! avec {
    // () => { Vec::new() };
    // $($element:expr),+   -- for $element:expr divided by ',' + ( at least one )
    // $(,)?                -- ',' can be there on  --> trailing ,
    ($($element:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::new();
        // Smiliar syntax
        $( // for array
            vs.push($element);
        )* // do repetition
        vs
    }};

    ($element:expr; $count:expr) => {{
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        vs.resize(count, $element);
        //vs.extend(::std::iter::repeat($element).take(count));
        vs
    }};

}

trait MaxValue {
    fn max_value() -> Self;
}

macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

max_impl!(i32);

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
    let x: Vec<&'static str> = avec!["first string", "second string",];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], "first string");
    assert_eq!(x[1], "second string");
}

#[test]
fn max_value_test() {
    let max: i32 = MaxValue::max_value();
    assert_eq!(max, <i32>::MAX);
}

#[test]
fn clone_2() {
    let x: Vec<i32> = avec![42; 2];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
    let x: Vec<i32> = avec!(42; 12);
    assert_eq!(x.len(), 12);
}

#[test]
fn clone_2_nonliteral() {
    let mut y = Some(42);
    let x: Vec<i32> = avec![y.take().unwrap(); 2];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
    let x: Vec<i32> = avec!(42; 12);
    assert_eq!(x.len(), 12);
}
