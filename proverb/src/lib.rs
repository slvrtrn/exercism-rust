pub fn build_proverb(list: &[&str]) -> String {
    let length = list.len();
    if length < 1 {
        return String::new();
    }
    let mut pieces = Vec::with_capacity(length);
    for i in 0..(length - 1) {
        let piece = format!("For want of a {} the {} was lost.", list[i], list[i + 1]);
        pieces.push(piece)
    }
    pieces.push(format!("And all for the want of a {}.", list[0]));
    pieces.join("\n")
}
