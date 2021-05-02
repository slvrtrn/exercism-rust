/// Determines whether the supplied string is a valid ISBN number
/// (x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5 + x7 * 4 + x8 * 3 + x9 * 2 + x10 * 1) mod 11 == 0
pub fn is_valid_isbn(isbn: &str) -> bool {
    let sanitized = isbn.replace('-', "");
    return if sanitized.len() != 10 {
        false
    } else {
        let mut sum = 0;
        let mut chars: Vec<char> = sanitized.chars().collect();
        let last = chars.last();
        match last {
            Some(l) if *l == 'X' => sum += 10,
            Some(l) if l.is_ascii_digit() => sum += l.to_digit(10).unwrap(),
            _ => return false,
        }
        chars.remove(chars.len() - 1);
        for (i, c) in chars.iter().enumerate() {
            if c.is_ascii_digit() {
                sum += c.to_digit(10).unwrap() * (10 - i as u32)
            } else {
                return false;
            }
        }
        return sum % 11 == 0;
    };
}
