use series::*;

#[test]
fn test_with_zero_length() {
    let expected = vec!["".to_string(); 6];
    assert_eq!(series("92017", 0), expected);
}

#[test]
fn test_with_length_2() {
    let expected = vec![
        "92".to_string(),
        "20".to_string(),
        "01".to_string(),
        "17".to_string(),
    ];
    assert_eq!(series("92017", 2), expected);
}

#[test]
fn test_with_numbers_length() {
    let expected = vec!["92017".to_string()];
    assert_eq!(series("92017", 5), expected);
}

#[test]
fn test_too_long() {
    let expected: Vec<String> = vec![];
    assert_eq!(series("92017", 6), expected);
}

#[test]
fn test_with_length_1() {
    let expected = vec![
        "9".to_string(),
        "2".to_string(),
        "0".to_string(),
        "1".to_string(),
        "7".to_string()
    ];
    assert_eq!(series("92017", 1), expected);
}