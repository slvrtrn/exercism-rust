pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for x in 1..limit {
        for f in factors {
            if *f == 0 { continue; }
            if x % f == 0 {
                sum += x;
                break;
            }
        }
    }
    sum
}
