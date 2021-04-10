pub fn nth(n: u32) -> u32 {
    let numbers: Vec<u32> = (2..1_000_000)
        .filter(is_prime)
        .take((n + 1) as usize)
        .collect();
    return numbers[n as usize];
}

fn is_prime(x: &u32) -> bool {
    let upper = ((*x as f32).sqrt() as u32) + 1;
    for i in 2 .. upper {
        if x % i == 0 {
            return false
        }
    }
    return true;
}
