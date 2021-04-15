use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: BTreeMap<char, i32> = BTreeMap::new();
    for (k, v) in h.into_iter() {
        for c in v.into_iter() {
            let lowercase = c.to_ascii_lowercase();
            result.insert(lowercase, *k);
        }
    }
    result
}
