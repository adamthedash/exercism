pub fn annotate(minefield: &[&str]) -> Vec<String> {
    println!("{:?}", minefield);


    let mut output = vec![String::new(); minefield.len()];
    for (i, row) in minefield.iter().enumerate() {
        let row_start = if i > 0 { i - 1 } else { 0 };
        let row_end = if i < minefield.len() - 1 { i + 2 } else { minefield.len() };

        for (j, c) in row.chars().enumerate() {
            if c == '*' {
                output[i].push('*');
                continue;
            }

            let col_start = if j > 0 { j - 1 } else { 0 };
            let col_end = if j < row.len() - 1 { j + 2 } else { row.len() };

            // Accumulate mines in local neighbourhood
            let mut val = 0;
            for k in row_start..row_end {
                for l in col_start..col_end {
                    let ch = minefield[k].chars().nth(l).unwrap();
                    println!("	{} {} {}", k, l, ch);

                    if ch == '*' { val += 1 }
                }
            }


            if val == 0 {
                output[i].push_str(" ")
            } else {
                output[i].push_str(&val.to_string())
            }
        }
    }

    return output;
}
