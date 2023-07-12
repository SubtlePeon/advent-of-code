pub fn part_one(input: &str) -> i32 {
    let mut max = vec![];
    let mut so_far = 0;
    for line in input.lines() {
        if line == "" {
            max.push(so_far);
            max.sort_by(|a, b| b.cmp(a));
            while max.len() > 1 {
                max.pop();
            }
            so_far = 0;
        } else {
            so_far += line.parse::<i32>().unwrap();
        }
    }

    max[0]
}

/// Copy and pasted from part one with slight tweaks
pub fn part_two(input: &str) -> i32 {
    let mut max = vec![];
    let mut so_far = 0;
    for line in input.lines() {
        if line == "" {
            max.push(so_far);
            max.sort_by(|a, b| b.cmp(a));
            while max.len() > 3 {
                max.pop();
            }
            so_far = 0;
        } else {
            so_far += line.parse::<i32>().unwrap();
        }
    }

    max.into_iter().sum()
}
