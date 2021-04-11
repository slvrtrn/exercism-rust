pub fn series(digits: &str, len: usize) -> Vec<String> {
    let digits_len = digits.len();
    return match len {
        0 | _ if len > digits_len => Vec::new(),
        _ => {
            let mut result: Vec<String> = Vec::new();
            let upper = digits_len - len;
            for i in 0..=upper {
                let str = digits[i..(i + len)].to_owned();
                result.push(str)
            }
            result
        }
    };
}
