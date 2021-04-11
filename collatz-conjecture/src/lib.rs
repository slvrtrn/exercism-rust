pub fn collatz(n: u64) -> Option<u64> {
    return match n {
        0 => None,
        1 => Some(0),
        _ => {
            let mut count = 0;
            let mut x = n;
            loop {
                count += 1;
                if x % 2 == 0 {
                    x /= 2;
                    if x == 1 {
                        return Some(count);
                    }
                } else {
                    x = x * 3 + 1;
                }
            }
        }
    };
}
