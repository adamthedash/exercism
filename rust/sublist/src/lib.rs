use std::iter::zip;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn eq<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    for (x, y) in zip(a, b) {
        if x != y {
            return false;
        }
    }
    return true;
}

fn le<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() > b.len() {
        return false;
    }

    for j in 0..(b.len() - a.len() + 1) {
        if eq(a, &b[j..j + a.len()]) {
            return true;
        }
    }

    return false;
}

pub fn sublist<T: PartialEq + std::fmt::Debug>(a: &[T], b: &[T]) -> Comparison {
    println!("{:?} {:?} {} {}", a, b, le(a, b), le(b, a));

    return match (le(a, b), le(b, a)) {
        (true, true) => { Comparison::Equal }
        (false, true) => { Comparison::Superlist }
        (true, false) => { Comparison::Sublist }
        (false, false) => { Comparison::Unequal }
    };
}
