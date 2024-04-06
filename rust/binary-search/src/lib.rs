use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut i = 0;
    let mut j = array.len();

    while i < j {
        let guess = (i + j) / 2;
        match key.cmp(&array[guess]) {
            Ordering::Equal => { return Some(guess); }
            Ordering::Less => { j = guess; }
            Ordering::Greater => { i = guess + 1; }
        }
    }

    return None;
}
