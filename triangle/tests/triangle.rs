use rstest::*;

use triangle::*;

#[test]
fn positive_length_sides_are_ok() {
    let sides = [2, 2, 2];
    let triangle = Triangle::build(sides);
    assert!(triangle.is_some());
}

#[rstest]
#[case::zero_length_sides_are_illegal([0, 0, 0])]
#[case::one_length_zero_side_first([0, 2, 2])]
#[case::one_length_zero_side_second([2, 0, 2])]
#[case::one_length_zero_side_third([2, 2, 0])]
#[case::sum_of_two_sides_must_equal_or_exceed_the_remaining_side_one([7, 3, 2])]
#[case::sum_of_two_sides_must_equal_or_exceed_the_remaining_side_two([1, 1, 3])]
fn invalid_sides(#[case] sides: [u64; 3]) {
    let triangle = Triangle::build(sides);
    assert!(triangle.is_none());
}

#[rstest]
#[case([2, 2, 2])]
#[case([10, 10, 10])]
fn equilateral_triangles(#[case] sides: [u64; 3]) {
    let triangle = Triangle::build(sides).unwrap();
    assert!(triangle.is_equilateral());
    assert!(!triangle.is_scalene());
    assert!(!triangle.is_degenerate())
}

#[rstest]
#[case([3, 4, 4])]
#[case([4, 4, 3])]
#[case([4, 3, 4])]
#[case([4, 7, 4])]
fn isosceles_triangles(#[case] sides: [u64; 3]) {
    let triangle = Triangle::build(sides).unwrap();
    assert!(!triangle.is_equilateral());
    assert!(triangle.is_isosceles());
    assert!(!triangle.is_scalene());
    assert!(!triangle.is_degenerate())
}

#[rstest]
#[case([3, 4, 5])]
#[case([5, 4, 6])]
#[case([10, 11, 12])]
#[case([5, 4, 2])]
fn scalene_triangles(#[case] sides: [u64; 3]) {
    let triangle = Triangle::build(sides).unwrap();
    assert!(!triangle.is_equilateral());
    assert!(!triangle.is_isosceles());
    assert!(triangle.is_scalene());
    assert!(!triangle.is_degenerate())
}

#[rstest]
#[case([3, 5, 8])]
#[case([8, 3, 5])]
#[case([8, 5, 3])]
fn degenerate_triangles(#[case] sides: [u64; 3]) {
    let triangle = Triangle::build(sides).unwrap();
    assert!(!triangle.is_equilateral());
    assert!(!triangle.is_isosceles());
    assert!(triangle.is_scalene());
    assert!(triangle.is_degenerate())
}

#[test]
#[cfg(feature = "generic")]
fn scalene_triangle_with_floating_point_sides() {
    let sides = [0.4, 0.6, 0.3];
    let triangle = Triangle::build(sides).unwrap();
    assert!(!triangle.is_equilateral());
    assert!(!triangle.is_isosceles());
    assert!(triangle.is_scalene());
}

#[test]
#[cfg(feature = "generic")]
fn equilateral_triangles_with_floating_point_sides() {
    let sides = [0.2, 0.2, 0.2];
    let triangle = Triangle::build(sides).unwrap();
    assert!(triangle.is_equilateral());
    assert!(!triangle.is_scalene());
}

#[test]
#[cfg(feature = "generic")]
fn isosceles_triangle_with_floating_point_sides() {
    let sides = [0.3, 0.4, 0.4];
    let triangle = Triangle::build(sides).unwrap();
    assert!(!triangle.is_equilateral());
    assert!(triangle.is_isosceles());
    assert!(!triangle.is_scalene());
}

#[test]
fn degenerate_triangle_with_floating_point_sides() {
    let sides = [0.4, 0.7, 1.1];
    let triangle = Triangle::build(sides).unwrap();
    assert!(!triangle.is_equilateral());
    assert!(!triangle.is_isosceles());
    assert!(triangle.is_scalene());
    assert!(triangle.is_degenerate());
}

#[rstest]
#[cfg(feature = "generic")]
#[case([0.0, 0.4, 0.3])]
#[case([0.1, 0.3, 0.5])]
fn invalid_triangle_with_floating_point_sides(#[case] sides: [f64; 3]) {
    let triangle = Triangle::build(sides);
    assert!(triangle.is_none());
}
