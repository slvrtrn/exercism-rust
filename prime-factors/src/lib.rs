fn is_prime(x: &u64) -> bool {
    let upper = ((*x as f64).sqrt() as u64) + 1;
    for i in 2..upper {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}


pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }
    let primes = (2..=n).filter(is_prime).into_iter();
    let mut result: Vec<u64> = vec![];
    let mut current = n;
    for p in primes {
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
