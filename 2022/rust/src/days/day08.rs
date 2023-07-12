pub fn part_one(input: &str) -> u32 {
    let mut data = Vec::with_capacity(99);
    // Put all data in array
    for row in input.lines() {
        let mut row_data = Vec::with_capacity(99);
        for byte in row.bytes() {
            row_data.push(byte - b'0');
        }
        data.push(row_data);
    }

    let rows = data.len();
    let columns = data.get(0).unwrap().len();

    let mut viewable = 0;
    for r in 0..rows {
        for c in 0..columns {
            let height = data[r][c];
            // Check left
            let left_blocked = (0..c).map(|col| data[r][col] >= height).any(|x| x);
            let right_blocked = (c + 1..columns).map(|col| data[r][col] >= height).any(|x| x);
            let top_blocked = (0..r).map(|row| data[row][c] >= height).any(|x| x);
            let bottom_blocked = (r + 1..rows).map(|row| data[row][c] >= height).any(|x| x);

            if !left_blocked || !right_blocked || !top_blocked || !bottom_blocked {
                viewable += 1;
            }
        }
    }

    viewable
}

pub fn part_two(input: &str) -> u32 {
    let mut data = Vec::with_capacity(99);
    // Put all data in array
    for row in input.lines() {
        let mut row_data = Vec::with_capacity(99);
        for byte in row.bytes() {
            row_data.push(byte - b'0');
        }
        data.push(row_data);
    }

    let rows = data.len();
    let columns = data.get(0).unwrap().len();

    let mut _maxr = 0;
    let mut _maxc = 0;
    let mut maxval = 0;

    for r in 0..rows {
        for c in 0..columns {
            let height = data[r][c];
            // Check left
            let left_view: u32 = (0..c).rev().scan(false, |st, col| if !*st { *st = data[r][col] >= height; Some(1) } else { None }).sum();
            let right_view: u32 = (c + 1..columns).scan(false, |st, col| if !*st { *st = data[r][col] >= height; Some(1) } else { None }).sum();
            let top_view: u32 = (0..r).rev().scan(false, |st, row| if !*st { *st = data[row][c] >= height; Some(1) } else { None }).sum();
            let bottom_view: u32 = (r + 1..rows).scan(false, |st, row| if !*st { *st = data[row][c] >= height; Some(1) } else { None }).sum();

            let view = left_view * right_view * top_view * bottom_view;
            if view > maxval {
                _maxr = r;
                _maxc = c;
                maxval = view;
            }
        }
    }

    maxval
}
