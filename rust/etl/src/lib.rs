use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut out = BTreeMap::new();
    for (&val, keys) in h{
        for &key in keys {
            println!("{} {}", key, val);
            out.insert(key.to_ascii_lowercase(), val);
        }
    }

    return out;
}
