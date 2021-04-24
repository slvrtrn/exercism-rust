use queen_attack::*;
use rstest::*;

#[test]
fn chess_position_on_the_board_is_some() {
    assert!(ChessPosition::new(2, 4).is_some());
}

#[rstest]
#[case(- 1, 2)]
#[case(8, 2)]
#[case(5, - 1)]
#[case(5, 8)]
fn chess_position_off_the_board_is_none(#[case] rank: i32, #[case] file: i32) {
    assert!(ChessPosition::new(rank, file).is_none());
}

#[test]
fn queen_is_created_with_a_valid_position() {
    Queen::new(ChessPosition::new(2, 4).unwrap());
}

#[test]
fn queens_that_can_not_attack() {
    let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
    let black_queen = Queen::new(ChessPosition::new(6, 6).unwrap());

    assert!(!white_queen.can_attack(&black_queen));
}

#[rstest]
#[case(2, 4, 2, 6)]
#[case(4, 5, 3, 5)]
#[case(2, 2, 0, 4)]
#[case(2, 2, 3, 1)]
#[case(2, 2, 5, 5)]
fn queens_on_the_same_rank_can_attack(
    #[case] rank1: i32,
    #[case] file1: i32,
    #[case] rank2: i32,
    #[case] file2: i32,
) {
    let white_queen = Queen::new(ChessPosition::new(rank1, file1).unwrap());
    let black_queen = Queen::new(ChessPosition::new(rank2, file2).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}
