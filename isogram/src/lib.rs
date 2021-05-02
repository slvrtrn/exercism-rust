use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut known_chars: HashSet<char> = HashSet::new();
    for c in candidate.chars() {
        if !c.is_ascii_alphabetic() {
            continue;
        }
        if !known_chars.insert(c.to_ascii_lowercase()) {
            return false;
        }
    }
    return true;
}
