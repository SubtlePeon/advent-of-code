use std::collections::VecDeque;

pub fn part_one(input: &str) -> String {
    // hard coding some values b/c easier
    let mut lines = input.lines();
    let mut stacks = Vec::with_capacity(9);
    for _ in 0..9 {
        stacks.push(VecDeque::new());
    }

    for _ in 0..8 {
        let line = lines.next().unwrap();
        for i in 0..9 {
            if &line[4 * i..=4 * i + 2] != "   " {
                stacks[i].push_front(&line[4 * i + 1 ..= 4 * i + 1]);
            }
        }
    }

    for _ in 0..2 { let _ = lines.next().unwrap(); }

    for line in lines {
        let inst = line.split(" ").collect::<Vec<_>>();
        let count = inst[1].parse::<i32>().unwrap();
        let from = inst[3].parse::<usize>().unwrap();
        let to = inst[5].parse::<usize>().unwrap();

        for _ in 0..count {
            let item = stacks[from - 1].pop_back().unwrap();
            stacks[to - 1].push_back(item);
        }
    }

    let mut res = String::new();
    for stack in &stacks {
        res.push_str(&stack[stack.len() - 1]);
    }

    res
}

pub fn part_two(input: &str) -> String {
    // hard coding some values b/c easier
    let mut lines = input.lines();
    let mut stacks = Vec::with_capacity(9);
    for _ in 0..9 {
        stacks.push(VecDeque::new());
    }

    for _ in 0..8 {
        let line = lines.next().unwrap();
        for i in 0..9 {
            if &line[4 * i..=4 * i + 2] != "   " {
                stacks[i].push_front(&line[4 * i + 1 ..= 4 * i + 1]);
            }
        }
    }

    for _ in 0..2 { let _ = lines.next().unwrap(); }

    for line in lines {
        let inst = line.split(" ").collect::<Vec<_>>();
        let count = inst[1].parse::<usize>().unwrap();
        let from = inst[3].parse::<usize>().unwrap();
        let to = inst[5].parse::<usize>().unwrap();

        let len = stacks[from - 1].len();
        let mut items = stacks[from - 1].drain(len - count .. len).collect::<VecDeque<_>>();
        stacks[to - 1].append(&mut items);
    }

    let mut res = String::new();
    for stack in &stacks {
        res.push_str(&stack[stack.len() - 1]);
    }

    res
}
