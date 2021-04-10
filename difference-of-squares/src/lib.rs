pub fn square_of_sum(n: u32) -> u32 {
    if n < 1 {
        return 0;
    }
    let sum: u32 = (1..=n).sum();
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    if n < 1 {
        return 0;
    }
    (1..=n).map(|x| x.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    let square = square_of_sum(n);
    let sum = sum_of_squares(n);
    square - sum
}
