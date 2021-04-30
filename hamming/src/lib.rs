/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        None
    } else {
        Some(s1.chars().zip(s2.chars()).fold(0, |acc, (c1, c2)| {
            if c1 != c2 {
                acc + 1
            } else {
                acc
            }
        }))
    }
}
