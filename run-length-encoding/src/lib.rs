pub fn encode(source: &str) -> String {
    let mut result = String::new();
    return if source.is_empty() {
        result
    } else {
        let mut chars = source.chars();
        let mut current_char: Option<char> = chars.next();
        let mut current_count: usize = 1;

        fn format_chunk(count: usize, ch: char) -> String {
            return if count > 1 {
                format!("{}{}", count, ch)
            } else {
                ch.to_string()
            };
        }

        for ch in chars {
            match current_char {
                Some(current_ch) if current_ch == ch => {
                    current_count += 1;
                }
                Some(current_ch) => {
                    result += &format_chunk(current_count, current_ch);
                    current_char = Some(ch);
                    current_count = 1;
                }
                _ => {}
            }
        }
        match current_char {
            Some(current_ch) => result += &format_chunk(current_count, current_ch),
            _ => {}
        }
        result
    };
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    return if source.is_empty() {
        result
    } else {
        let mut current_count: String = String::new();
        for ch in source.chars() {
            if ch.is_ascii_digit() {
                current_count.push(ch);
            } else {
                if current_count.is_empty() {
                    result.push(ch);
                } else {
                    match current_count.parse::<usize>() {
                        Ok(count) => {
                            for _ in 0..count { result.push(ch) };
                            current_count.clear();
                        }
                        _ => {
                            panic!("Could not parse count: {}", current_count);
                        }
                    }
                }
            }
        }
        result
    };
}
