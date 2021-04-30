pub fn score(word: &str) -> u64 {
    word.chars().fold(0, |acc, c| {
        match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => acc + 1,
            'D' | 'G' => acc + 2,
            'B' | 'C' | 'M' | 'P' => acc + 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => acc + 4,
            'K' => acc + 5,
            'J' | 'X' => acc + 8,
            'Q' | 'Z' => acc + 10,
            _ => acc
        }
    })
}
