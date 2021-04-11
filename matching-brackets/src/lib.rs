pub fn brackets_are_balanced(string: &str) -> bool {
    let mut expected_braces: Vec<char> = vec![];
    for c in string.chars() {
        match c {
            '{' => expected_braces.push('}'),
            '[' => expected_braces.push(']'),
            '(' => expected_braces.push(')'),
            '}' | ']' | ')' => {
                // .pop should also work and probably is better
                match expected_braces.last() {
                    Some(ch) if *ch == c => {
                        expected_braces.remove(expected_braces.len() - 1);
                    }
                    _ => {
                        return false;
                    }
                }
            }
            _ => {}
        };
    }
    expected_braces.is_empty()
}
