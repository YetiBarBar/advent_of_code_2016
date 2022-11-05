fn step(in_data: &str) -> String {
    let chrs: Vec<char> = in_data.chars().collect();
    chrs.iter()
        .chain(std::iter::once(&'0'))
        .chain(
            chrs.iter()
                .rev()
                .map(|chr| if chr == &'0' { &'1' } else { &'0' }),
        )
        .collect()
}

fn checksum(data: &str) -> String {
    let chrs: Vec<char> = data.chars().collect();
    let mut chk = chrs;
    loop {
        chk = chk
            .chunks_exact(2)
            .map(|chunk| if chunk[0] == chunk[1] { '1' } else { '0' })
            .collect();
        if chk.len() % 2 == 1 {
            return chk.iter().collect();
        }
    }
}

fn main() {
    let input = "00101000101111010";
    println!("Step 1: {}", compute(input, 272));
    println!("Step 2: {}", compute(input, 35_651_584));
}

fn compute<T: Into<String>>(data: T, len: usize) -> String {
    let mut input: String = data.into();
    while input.len() < len {
        input = step(&input);
    }
    let input: String = input.chars().take(len).collect();
    checksum(&input)
}
