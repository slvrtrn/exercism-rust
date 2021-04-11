pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }
    let mut alphabetical_count = 0;
    let mut uppercase_count = 0;
    for c in trimmed.chars() {
        if c.is_alphabetic() {
            alphabetical_count += 1;
            if c.is_ascii_uppercase() {
                uppercase_count += 1;
            }
        }
    }
    if trimmed.ends_with("?") {
        return if uppercase_count == alphabetical_count && alphabetical_count > 0 {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        };
    }
    return if uppercase_count == alphabetical_count && alphabetical_count > 0 {
        "Whoa, chill out!"
    } else {
        "Whatever."
    };
}
