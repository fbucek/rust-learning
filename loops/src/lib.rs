#[derive(Default)]
pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment count 
        self.count += 1;

        // Return increment until count < 6
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
    fn test_default() {
        let counter = Counter::default();
        assert_eq!(counter.count, 0);
    }

    #[test]
    fn test_new() {
        let counter = Counter::new();
        assert_eq!(counter.count, 0);
    }

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

    #[test]
    fn test_iters() {
        let all: Vec<u32> = Counter::new().collect();
        assert_eq!(&vec![1,2,3,4,5], &all);
        let skip: Vec<u32> = Counter::new().skip(1).collect();
        assert_eq!(&vec![2,3,4,5], &skip);
        
        
        let zip = &all.iter().zip(&skip);
        let (unzip_a, unzip_b): (Vec<u32>, Vec<u32>) = zip.clone().unzip();
        assert_eq!(&vec![1,2,3,4], &unzip_a);
        assert_eq!(&vec![2,3,4,5], &unzip_b);
        
        let map: Vec<u32> = all.iter().zip(Counter::new().skip(1)).map(|(a, b)| a + b).collect();
        assert_eq!(&vec![3, 5, 7, 9], &map);

        // let iterzip = Counter::new().zip(Counter::new().skip(1));
        //assert_eq!()
    }
}