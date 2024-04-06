/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // Check for valid chars
    if !code.chars().all(|x| x.is_ascii_digit() || x == ' ') {
        return false;
    }

    // Parse string
    let mut numbers: Vec<u32> = code.chars()
        .filter(|x| x.is_ascii_digit())
        .map(|x| x.to_digit(10).unwrap())
        .collect();


    if numbers.len() <= 1 {
        return false;
    }

    // luhn check
    for i in (0..numbers.len()).rev().skip(1).step_by(2) {
        println!("{}", i);
        // Double every 2nd digit in place
        numbers[i] *= 2;

        // Mod 9 ish
        if numbers[i] > 9 {
            numbers[i] -= 9
        }
    }

    let sum = numbers.iter().sum::<u32>();

    return sum % 10 == 0;
}
