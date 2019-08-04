struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zip_uneven_test() {
        let a1 = [1, 2];
        let a2 = [4, 5];

        let mut iter = a1.iter().zip(a2.iter());
        assert_eq!(iter.next(), Some((&1, &4)));
        assert_eq!(iter.next(), Some((&2, &5)));
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn filter_test() { 
        let a = [0i32, 1, 2];
        //assert!(&a[0].is_positive());

        let mut iter1 = a.iter().filter(|x| x.is_positive());

        assert_eq!(iter1.next(), Some(&1));
        assert_eq!(iter1.next(), Some(&2));
        assert_eq!(iter1.next(), None);

        let mut iter2 = a.iter().filter(|&x| *x > 1);
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(iter2.next(), None);


    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                    .map(|(a, b)| a * b)
                                    .filter(|x| x % 3 == 0)
                                    .sum();
        assert_eq!(18, sum);
    }
}