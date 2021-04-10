pub fn verse(n: u32) -> String {
    match n {
        0 => [
            "No more bottles of beer on the wall, no more bottles of beer.\n",
            "Go to the store and buy some more, 99 bottles of beer on the wall.\n"
        ].join(""),
        1 => [
            "1 bottle of beer on the wall, 1 bottle of beer.\n",
            "Take it down and pass it around, no more bottles of beer on the wall.\n"
        ].join(""),
        2 => [
            "2 bottles of beer on the wall, 2 bottles of beer.\n",
            "Take one down and pass it around, 1 bottle of beer on the wall.\n"
        ].join(""),
        n => [
            format!("{n} bottles of beer on the wall, {n} bottles of beer.\n", n = n),
            format!("Take one down and pass it around, {m} bottles of beer on the wall.\n", m = n - 1)
        ].join("")
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    let mut current = start;
    loop {
        result.push_str(&verse(current));
        if current == end {
            break;
        }
        result.push_str("\n");
        current -= 1;
    }
    return result;
}
