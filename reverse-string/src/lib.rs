pub fn reverse(input: &str) -> String {
    return if cfg!(feature = "grapheme") {
        use unicode_segmentation::UnicodeSegmentation;
        let mut result = Vec::new();
        for grapheme in input.graphemes(true).rev() {
            result.push(grapheme)
        }
        result.join("")
    } else {
        let mut result = String::with_capacity(input.len());
        for char in input.chars().rev() {
            result.push(char)
        }
        result
    }
}
