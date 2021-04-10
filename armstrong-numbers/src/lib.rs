fn split_to_digits(num: u32) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    let mut current = num;
    loop {
        if current == 0 { break; }
        numbers.push(current % 10);
        current /= 10;
    }
    numbers
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = split_to_digits(num);
    let digits_count = digits.len();
    let armstrong_sum: u32 = digits.iter().map(|x| x.pow(digits_count as u32)).sum();
    return armstrong_sum == num
}
