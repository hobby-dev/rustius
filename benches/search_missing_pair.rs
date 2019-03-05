#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use rustius::search_missing_pair;
    use test::Bencher;

    #[bench]
    fn test0(b: &mut Bencher) {
        let mut data = vec![12, 23, 12, -232, 12, 0, 23, 42, -232, 12];
        b.iter(|| search_missing_pair(&mut data));
    }

    #[bench]
    fn test1(b: &mut Bencher) {
        let mut data = vec![0, 0, 0, 0, 0, -232, -232, 0, 1, 0, 0, 0];
        b.iter(|| search_missing_pair(&mut data));
    }

    #[bench]
    fn test2(b: &mut Bencher) {
        let mut data = vec![1, 0, 0, 0, -232, 0, 0, -232, 1, 0, 0, 1];
        b.iter(|| search_missing_pair(&mut data));
    }

    #[bench]
    fn test3(b: &mut Bencher) {
        let mut data = vec![
            3211, 1, 123, 164, 7685, -232, 164, 123, 3211, 1, 7685, -232, 0, 1,
        ];
        b.iter(|| search_missing_pair(&mut data));
    }
}
