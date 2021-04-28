pub fn find<
    K: PartialOrd,
    C: AsRef<[K]>
>(array: C, key: K) -> Option<usize> {
    let array = array.as_ref();
    return match array {
        [] => None,
        [x] if *x == key => Some(0),
        [_] => None,
        _ => {
            let middle = array.len() / 2;
            let left_half = &array[0..middle];
            match left_half.last() {
                Some(x) if *x < key => {
                    let right_half = &array[middle..];
                    find(right_half, key).map(|i| i + middle)
                }
                Some(x) if *x > key => find(left_half, key),
                Some(x) if *x == key => Some(middle - 1),
                _ => None
            }
        }
    };
}
