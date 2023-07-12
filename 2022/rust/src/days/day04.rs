pub fn part_one(input: &str) -> i32 {
    let mut total = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        // split up each line into ranges
        let v = line.split(",").collect::<Vec<_>>();
        // useless checking
        if v.len() != 2 {
            panic!("Invalid input: {}", line);
        }

        let ranges = v
            .into_iter()
            .map(|r| {
                let r = r.split("-").collect::<Vec<_>>();
                // no useless checking this time
                r[0].parse::<i32>().unwrap()..=r[1].parse::<i32>().unwrap()
            })
            .collect::<Vec<_>>();

        if ranges[0].contains(ranges[1].start()) && ranges[0].contains(ranges[1].end())
            || ranges[1].contains(ranges[0].start()) && ranges[1].contains(ranges[0].end())
        {
            total += 1;
        }
    }
    total
}

pub fn part_two(input: &str) -> i32 {
    let mut total = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        // split up each line into ranges
        let v = line.split(",").collect::<Vec<_>>();
        // useless checking
        if v.len() != 2 {
            panic!("Invalid input: {}", line);
        }

        let ranges = v
            .into_iter()
            .map(|r| {
                let r = r.split("-").collect::<Vec<_>>();
                // no useless checking this time
                r[0].parse::<i32>().unwrap()..=r[1].parse::<i32>().unwrap()
            })
            .collect::<Vec<_>>();

        if ranges[0].contains(ranges[1].start())
            || ranges[0].contains(ranges[1].end())
            || ranges[1].contains(ranges[0].start())
            || ranges[1].contains(ranges[0].end())
        {
            total += 1;
        }
    }
    total
}
