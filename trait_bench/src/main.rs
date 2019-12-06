/// @see https://www.reddit.com/r/rust/comments/74llky/trait_objects_22x_slower_than_static_dispatch/


extern crate test;

const N: usize = 100;

trait A {
    fn f(&self) -> usize;
}

struct B;

struct C;

struct D;

struct E {
    val: usize
}

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

impl A for E {
    fn f(&self) -> usize {
        match self.val {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => unreachable!(),
        }
    }
}

fn method_0(e: &E) -> usize {
    return 0
}

fn method_1(e: &E) -> usize {
    return 1
}

fn method_2(e: &E) -> usize {
    return 2;
}

static E_METHODS: &[fn(e: &E) -> usize] = &[
    method_0,
    method_1,
    method_2,
];

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

    #[bench]
    fn my_idea_of_enum_dispatch(b: &mut Bencher) {
        let mut xs: Vec<E> = Vec::new();
        for i in 0..N {
            xs.push(E { val: 0 });
            xs.push(E { val: 1 });
            xs.push(E { val: 2 });
        }
        assert_eq!(xs.len(), N * 3);
        b.iter(|| {
            let mut sum = 0;
            for x in &xs {
                sum += black_box(match x.val {
                    0 => method_0(x),
                    1 => method_1(x),
                    2 => method_2(x),
                    _ => unreachable!(),
                });
            }
            sum
        });
    }

    #[bench]
    fn my_idea_of_dynamic_dispatch(b: &mut Bencher) {
        let mut xs: Vec<Box<E>> = Vec::new();
        for i in 0..N {
            xs.push(Box::new(E { val: 0 }));
            xs.push(Box::new(E { val: 1 }));
            xs.push(Box::new(E { val: 2 }));
        }
        assert_eq!(xs.len(), N * 3);
        b.iter(|| {
            let mut sum = 0;
            for x in &xs {
                sum += black_box(E_METHODS[x.val](x));
            }
            sum
        });
    }
}