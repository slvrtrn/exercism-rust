/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let lowercase = sentence.to_ascii_lowercase();
    ('a'..='z').all(|c| lowercase.contains(c))
}
