enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    return match size {
        0 => vec![],
        1 => vec![vec![1]],
        _ => {
            let mut result = vec![vec![0; size]; size];

            let mut top = 0;
            let mut left = 0;
            let mut bottom = size - 1;
            let mut right = size - 1;
            let mut direction = Direction::Right;
            let mut current = 1;

            while top <= bottom && left <= right {
                match direction {
                    Direction::Right => {
                        for i in left..=right {
                            result[top][i] = current;
                            current += 1;
                        }
                        top += 1;
                        direction = Direction::Down;
                    }
                    Direction::Down => {
                        for i in top..=bottom {
                            result[i][right] = current;
                            current += 1;
                        }
                        right -= 1;
                        direction = Direction::Left;
                    }
                    Direction::Left => {
                        for i in (left..=right).rev() {
                            result[bottom][i] = current;
                            current += 1;
                        }
                        bottom -= 1;
                        direction = Direction::Up;
                    }
                    Direction::Up => {
                        for i in (top..=bottom).rev() {
                            result[i][left] = current;
                            current += 1;
                        }
                        left += 1;
                        direction = Direction::Right;
                    }
                }
            }

            result
        }
    };
}
