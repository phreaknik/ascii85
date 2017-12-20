pub fn ascii85_decode(s: &str) -> String {
    // Calculate pad amount, such that input is divisible by 5
    let pad_count = (5 - s.len() % 5) % 5;

    // Split input into bytes, subtract 33
    let mut rad85: Vec<u8> = s.as_bytes().iter().map(|x| x - 33).collect();

    // Pad with max base85 value
    rad85.append(&mut vec![84; pad_count]);

    // Convert to 32b concatenated characters
    let mut concat: Vec<u32> = Vec::new();
    for w in rad85.chunks(5) {
        let mut temp: u32 = w[0] as u32;
        for i in 1..w.len() {
            temp *= 85;
            temp += w[i] as u32;
        }
        concat.push(temp);
    }

    // Split into bytes
    let mut bytes: Vec<u8> = Vec::new();
    for mut n in concat {
        // Calculate each of the 4 bytes
        for _ in 0..4 {
            bytes.push(((n & 0xFF000000) >> 24) as u8);
            n = n << 8;
        }
    }

    // Remove padded characters
    let corrected_length = bytes.len() - pad_count;
    bytes.truncate(corrected_length);

    return String::from_utf8(bytes).unwrap_or("".to_string());
}

pub fn ascii85_encode(s: &str) -> String {
    // Calculate pad amount, such that input is divisible by 4
    let pad_count = (4 - s.len() % 4) % 4;

    // Concatenate bytes into 32 bit numbers (groups of 4)
    let mut nums: Vec<u32> = Vec::new();
    for w in s.as_bytes().chunks(4) {
        // Concatenate bytes in groups of 4
        let concat = w.iter().fold(0, |acc: u32, &x| (acc << 8) + (x as u32));

        // Pad input if not a multiple of 4
        let concat = if w.len() < 4 {
            concat * (256 as u32).pow(pad_count as u32)
        } else {
            concat
        };

        // Push new 32b number
        nums.push(concat);
    }

    // Divide 32b numbers into radix 85 numbers
    let mut rad85: Vec<u8> = Vec::new();
    for mut x in nums {
        let mut temp: Vec<u8> = Vec::new();
        temp.push((x % 85 + 33) as u8);
        for _ in 0..4 {
            x /= 85;
            temp.push((x % 85 + 33) as u8);
        }

        // Fix endian-ness
        temp.reverse();

        // Append to rad85
        rad85.append(&mut temp);
    }

    // Remove padded characters
    let corrected_length = rad85.len() - pad_count;
    rad85.truncate(corrected_length);

    return String::from_utf8(rad85).unwrap_or("".to_string());
}
