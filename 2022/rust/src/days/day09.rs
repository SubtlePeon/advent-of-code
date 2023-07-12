use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    visited.insert(tail_pos);
    for line in input.lines() {
        let mut line_split = line.split(" ");
        if let (Some(dir), Some(amt)) = (line_split.next(), line_split.next()) {
            let amt = amt.parse::<i32>().expect("Invalid amount");
            match dir {
                "L" => head_pos.0 -= amt,
                "R" => head_pos.0 += amt,
                "U" => head_pos.1 += amt,
                "D" => head_pos.1 -= amt,
                _ => panic!("Invalid dir: {}", &dir),
            }
            // Check if tail_pos is too far
            let mut x_diff = head_pos.0 - tail_pos.0;
            let mut y_diff = head_pos.1 - tail_pos.1;
            while x_diff.abs().max(y_diff.abs()) > 1 {
                tail_pos.0 += x_diff.signum();
                tail_pos.1 += y_diff.signum();
                // Add to visited
                visited.insert(tail_pos);
                // Get differences again
                x_diff = head_pos.0 - tail_pos.0;
                y_diff = head_pos.1 - tail_pos.1;
            }
        } else {
            panic!("Parsing went wrong: {}", &line);
        }
    }
    visited.len()
}

pub fn part_two(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut rope = vec![(0, 0); 10];
    visited.insert(*rope.last().unwrap());
    for line in input.lines() {
        let mut line_split = line.split(" ");
        if let (Some(dir), Some(amt)) = (line_split.next(), line_split.next()) {
            let amt = amt.parse::<i32>().expect("Invalid amount");
            match dir {
                "L" => rope.first_mut().unwrap().0 -= amt,
                "R" => rope.first_mut().unwrap().0 += amt,
                "U" => rope.first_mut().unwrap().1 += amt,
                "D" => rope.first_mut().unwrap().1 -= amt,
                _ => panic!("Invalid dir: {}", &dir),
            }
            for i in 1..10 {
                let head = rope[i - 1];
                let knot = &mut rope[i];
                let mut x_diff = head.0 - knot.0;
                let mut y_diff = head.1 - knot.1;
                while x_diff.abs().max(y_diff.abs()) > 1 {
                    knot.0 += x_diff.signum();
                    knot.1 += y_diff.signum();
                    // Add to visited
                    if i == 9 {
                        visited.insert(*knot);
                    }
                    // Get differences again
                    x_diff = head.0 - knot.0;
                    y_diff = head.1 - knot.1;
                }
            }
        } else {
            panic!("Parsing went wrong: {}", &line);
        }
    }
    display(&visited);
    visited.len()
}

fn display(visited: &HashSet<(i32, i32)>) {
    let x_max = visited.iter().max_by_key(|(a, _)| a).unwrap().0;
    let y_max = visited.iter().max_by_key(|(_, b)| b).unwrap().1;
    let x_min = visited.iter().min_by_key(|(a, _)| a).unwrap().0;
    let y_min = visited.iter().min_by_key(|(_, b)| b).unwrap().1;
    let mut lines = Vec::with_capacity((y_max - y_min) as usize);
    for r in y_min..y_max + 1 {
        let mut line = String::with_capacity((x_max - x_min) as usize);
        for c in x_min..x_max + 1 {
            let push_char = if visited.contains(&(c, r)) {
                '#'
            } else {
                '.'
            };
            line.push(push_char);
        }
        lines.push(line);
    }
    println!("{}", lines.into_iter().rev().collect::<Vec<_>>().join("\n"))
}
