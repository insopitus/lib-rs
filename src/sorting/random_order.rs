use rand::Rng;
// Fisher-Yates method

pub fn random_order<T>(arr: &mut [T]) -> &[T] {
    let len = arr.len();
    let mut rng = rand::thread_rng();
    for i in 0..len {
        let rn = rng.gen_range(i..len);
        arr.swap(i, rn);
    }
    arr
}

#[cfg(test)]
mod test {
    use super::random_order;

    #[test]
    fn it_should_work() {
        let mut input = vec![1, 2, 3, 4, 5];
        random_order(&mut input);
        assert!(input.len() == 5);
    }
    #[test]
    fn randomness_check() {
        const N: usize = 100000;
        let mut results = Vec::with_capacity(N);
        for _ in 0..N {
            let mut input = vec![1, 2, 3, 4, 5];
            random_order(&mut input);
            results.push(input);
        }
        let mut counts = [0; 5];
        for j in 0..N {
            let result = &results[j];
            for i in 0usize..5 {
                counts[i] += result[i]
            }
        }
        let average = N * 3;
        for item in counts {
            let diff = if item > average {
                item - average
            } else {
                average - item
            };
            assert!(diff < (N >> 5)); //3.125%
        }
    }
}
