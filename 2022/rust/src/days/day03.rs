pub fn part_1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let len = bytes.len();
        let first = &bytes[0 .. len / 2];
        let second = &bytes[len / 2 ..];
        let mut bits: u64 = 0;
        for byte in first {
            // b'z' would be 57
            bits |= 1u64 << (byte - b'A');
        }
        let mut common = None;
        for byte in second {
            if bits & (1u64 << (byte - b'A')) != 0 {
                common = Some(byte);
            }
        }
        let Some(letter) = common else { panic!("No common letter for line: {}", line) };
        if letter.is_ascii_uppercase() {
            total += (letter - b'A' + 27) as u32;
        } else {
            // should be lowercase due to input guarantees
            total += (letter - b'a' + 1) as u32;
        }
    }
    total
}

pub fn part_2(input: &str) -> u32 {
    let mut total = 0;
    // no itertools here
    let lines: Vec<_> = input.lines().collect();
    for chunk in lines.chunks(3) {
        // let-else statements are really nice
        let [first, second, third] = chunk else {
            panic!("Expected valid input");
        };
        
        let transform = |byte: u8| if byte.is_ascii_uppercase() {
            1u64 << (byte - b'A' + 27)
        } else {
            1u64 << (byte - b'a' + 1)
        };

        // can make a separate function but whatever
        let first_bits = first.bytes().fold(0u64, |acc, b| acc | transform(b));
        let second_bits = second.bytes().fold(0u64, |acc, b| acc | transform(b));
        let third_bits = third.bytes().fold(0u64, |acc, b| acc | transform(b));

        let all_bits = first_bits & second_bits & third_bits;

        // whoa this was actually kind of cool
        total += all_bits.trailing_zeros();
    }
    total
}
