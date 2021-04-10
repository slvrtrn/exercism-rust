pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }
    let mut result: Vec<u64> = vec![];
    let mut current = n;
    for p in 2..=n {
        loop {
            if current % p != 0 {
                break;
            }
            current /= p;
            result.push(p);
            if current == 1 {
                return result;
            }
        }
    }
    result
}
