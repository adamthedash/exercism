#[macro_export]
macro_rules! hashmap {
    () => {HashMap::new()};
    ($($k:expr => $v:expr),* $(,)?) => {
        {
            let mut h = HashMap::new();
            $(
                h.insert($k, $v);
            )*
            h
        }
    };
}
