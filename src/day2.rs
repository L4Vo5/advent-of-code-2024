use crate::utils::*;

pub fn part1(whole: &str) -> i64 {
    const SPACE: u8 = b' ';
    const NL: u8 = b'\n';
    const ZERO: u8 = b'0';
    assert!(SPACE > NL);
    assert!(b'0' > SPACE);
    let mut sum = 0;
    // ok so, the format seems to be either 1 or 2 digit numbers. Hmm.
    let bytes = whole.as_bytes();
    let mut i = 0;
    let mut prev: Option<u8> = None;
    let mut direction = 2;
    while i < bytes.len() {
        let first = unsafe {*bytes.get_unchecked(i)};
        let second = unsafe {*bytes.get_unchecked(i+1)};
        let num = if second > SPACE {
            i += 2;
            (first - ZERO) * 10 + (second - ZERO)
        } else {
            i += 1;
            first - ZERO
        };
        // note: i pointing at the next space (or nl, or eof)
        if let Some(actual_prev) = prev {
            let up = (num > actual_prev) as u8;
            let diff = if up == 1 {num - actual_prev} else {actual_prev - num};
            if diff > 0 && diff <= 3 {
                // ok: but does direction match?
                if direction as u8 == 2 {
                    direction = up;
                } else if direction != up {
                    // failure: rush through to the next line
                    prev = None;
                    direction = 2;
                    while i < bytes.len() && unsafe {*bytes.get_unchecked(i)} != NL {
                        i += 1;
                    }
                    i += 1;
                    if i >= bytes.len() {break;}
                    continue;
                }
            } else {
                // failure: rush through to the next line
                prev = None;
                direction = 2;
                while i < bytes.len() && unsafe {*bytes.get_unchecked(i)} != NL {
                    i += 1;
                }
                i += 1;
                if i >= bytes.len() {break;}
                continue;
            }
        }
        prev = Some(num);
        if i+1 >= bytes.len() {
            sum += 1;
            break;
        }
        if unsafe {*bytes.get_unchecked(i)} == NL {
            // we got here unharmed. Nice!
            prev = None;
            direction = 2;
            sum += 1;
            if i+1 >= bytes.len() {
                break;
            }
        }
        i += 1;
    }
    sum
}

/// AoC placement:
/// Time 00:07:26 rank 1410
pub fn part1_original(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    // let nums = get_numbers(&whole);
    let mut sum = 0;
    for line in lines {
        let nums = get_numbers(&line);
        let mut prev = nums[0].num;
        let increasing = nums[1].num > nums[0].num;
        let mut ok = true;
        for num in nums[1..].iter() {
            let n = num.num;
            if increasing {
                if (n - prev == 0) || ( n - prev > 3) {
                    ok = false
                }
            } else {
                if (prev - n == 0) || (prev - n > 3) {
                    ok = false;
                }
            }
            prev = n;
        }
        if ok {
            sum += 1;
        }
    }
    sum
}


/// AoC placement:
/// Time 00:14:04 rank 1578
pub fn part2(whole: &str) -> i64 {
    let lines = get_lines(&whole);
    // let nums = get_numbers(&whole);
    let mut sum = 0;
    for line in lines {
        let mut ok_final = false;
        let nums = get_numbers(&line);
        for i in 0..nums.len() {
            let mut mutated_line = nums.clone();
            mutated_line.remove(i);
            let mut prev = mutated_line[0].num;
            let increasing = mutated_line[1].num > mutated_line[0].num;
            let mut ok = true;
            for num in mutated_line[1..].iter() {
                let n = num.num;
                if increasing {
                    if (n - prev == 0) || ( n - prev > 3) {
                        ok = false
                    }
                } else {
                    if (prev - n == 0) || (prev - n > 3) {
                        ok = false;
                    }
                }
                prev = n;
            }
            if ok {
                ok_final = true;
            }
        }

        let mut prev = nums[0].num;
        let increasing = nums[1].num > nums[0].num;
        let mut ok = true;
        for num in nums[1..].iter() {
            let n = num.num;
            if increasing {
                if (n - prev == 0) || ( n - prev > 3) {
                    ok = false
                }
            } else {
                if (prev - n == 0) || (prev - n > 3) {
                    ok = false;
                }
            }
            prev = n;
        }
        if ok || ok_final {
            sum += 1;
        }
    }
    sum
}