//#![feature(test)]

extern crate test;

const N: usize = 100;

trait A {
    fn f(&self) -> usize;
}

struct B;

struct C;

struct D;

impl A for B { 
    fn f(&self) -> usize {
        let mut sum = 0; 
        for i in 1..N {
            sum += i;
        }
        sum
    }
}

impl A for C { 
    fn f(&self) -> usize {
        let mut sum = 0; 
        for i in 1..N {
            sum += i;
        }
        sum
    }
}

impl A for D { 
    fn f(&self) -> usize {
        let mut sum = 0; 
        for i in 1..N {
            sum += i;
        }
        sum
    }
}

enum F {
    B(B),
    C(C),
    D(D)
}

impl A for F {
    fn f(&self) -> usize {
        match self {
            &F::B(ref b) => b.f(),
            &F::C(ref c) => c.f(),
            &F::D(ref d) => d.f(),
        }
    }
}

// added this so it will compile at playground, however code
// would need to be "run" with `$ cargo bench` 
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn using_enum_dispatch(b: &mut Bencher) {
        let mut xs: Vec<F> = Vec::new();
        for _ in 0..N {
            xs.push(F::B(B));
            xs.push(F::C(C));
            xs.push(F::D(D));
        }
        assert_eq!(xs.len(), N * 3);
        b.iter(|| {
            let mut sum = 0;
            for x in &xs {
                sum += black_box(x.f());
            }
            sum
        });
    }

    #[bench]
    fn using_box_dispatch(b: &mut Bencher) {
        let mut xs: Vec<Box<A>> = Vec::new();
        for _ in 0..N {
            xs.push(Box::new(B));
            xs.push(Box::new(C));
            xs.push(Box::new(D));
        }
        assert_eq!(xs.len(), N * 3);
        b.iter(|| {
            let mut sum = 0;
            for x in &xs {
                sum += black_box(x.f());
            }
            sum
        });
    }

}