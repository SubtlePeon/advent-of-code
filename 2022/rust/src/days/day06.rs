const AMT: usize = 100;

pub fn part_one(input: &str) -> usize {
    for i in AMT..input.len() {
        if !dupl(&input[i - AMT ..= i]) {
            println!("{}", &input[i - AMT ..= i]);
            // Since i is 0-indexed, add one since the problem is 1-indexed
            return i + 1;
        }
    }
    unreachable!("no solution found");
}

fn dupl(input: &str) -> bool {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j { continue; }
            if input[i..=i] == input[j..=j] {
                return true;
            }
        }
    }
    false
}
