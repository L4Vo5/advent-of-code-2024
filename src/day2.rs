#![allow(dead_code)]
use crate::utils::*;

// roughly 122ms on my system (10000 runs)
pub fn part1(whole: &str) -> i64 {
    const SPACE: u16 = b' ' as u16;
    const NL: u16 = b'\n' as u16;
    const ZERO: u16 = b'0' as u16;
    let mut sum = 0;
    // ok so, the format seems to be either 1 or 2 digit numbers. Hmm.
    let bytes = whole.as_bytes();
    let mut i = 0;
    // each iteration is one LINE
    while i < bytes.len() {
        // Assumption: each line has AT LEAST three numbers. otherwise part 2 makes no sense.
        // So, the first 5 bytes are guaranteed to exist
        let first = unsafe {*bytes.get_unchecked(i)} as u16;
        let second = unsafe {*bytes.get_unchecked(i+1)} as u16;

        let num1 = if second > SPACE {
            // 2-digit number
            i += 3;
            // (first - ZERO) * 10 + (second - ZERO)
            // (a - c) * 10 + (b - c) * 1 = 
            // a * 10 + b - 11 * c
            first * 10 + second - 11 * ZERO
        } else {
            // 1-digit number
            i += 2;
            first - ZERO
        };
        // note: i pointing at the start of number 2.
        let first = unsafe {*bytes.get_unchecked(i)} as u16;
        let second = unsafe {*bytes.get_unchecked(i+1)} as u16;

        let num2 = if second > SPACE {
            i += 3;
            first * 10 + second - 11 * ZERO
        } else {
            i += 2;
            first - ZERO
        };
        // note: i pointing at the start of number 3.
        if num2 > num1 {
            // if the numbers are equal, this will wrap.
            let diff = (num2 - num1).wrapping_sub(1);
            if diff > 2 {
                // failure: go to next line.
                while i < bytes.len() && bytes[i] as u16 != NL {
                    i += 1;
                }
                i += 1;
                if i >= bytes.len() {break;}
                continue;
            }
            // start looping, looking for increasing order.
            // each iteration, i is pointing at the start of the number.
            let mut prev = num2;
            loop {
                let first = unsafe {*bytes.get_unchecked(i)} as u16;
                debug_assert!(first != NL);
                debug_assert!(first != SPACE);
                let second = unsafe {*bytes.get_unchecked(i+1)} as u16;
        
                let num = if second > SPACE {
                    i += 2;
                    first * 10 + second - 11 * ZERO
                } else {
                    i += 1;
                    first - ZERO
                };
                // if prev >= num, this will be huge and not pass the next test
                let diff = num.wrapping_sub(prev+1);
                if diff > 2 {
                    // failure: go to next line.
                    while i < bytes.len() && bytes[i] as u16 != NL {
                        i += 1;
                    }
                    i += 1;
                    break;
                }
                // i will be pointing at a space, NL, or EOF. if NL or EOF, exit this loop, and also, gg the line was fine!
                if i >= bytes.len() || bytes[i] as u16 == NL {
                    sum += 1;
                    i += 1;
                    break;
                }
                i += 1;
                prev = num;
            }
        } else {
            // if the numbers are equal, this will wrap.
            let diff = (num1 - num2).wrapping_sub(1);
            if diff > 2 {
                // failure: go to next line.
                while i < bytes.len() && bytes[i] as u16 != NL {
                    i += 1;
                }
                i += 1;
                if i >= bytes.len() {break;}
                continue;
            }
            // start looping, looking for decreasing order.
            // each iteration, i is pointing at the start of the number.
            let mut prev = num2;
            loop {
                let first = unsafe {*bytes.get_unchecked(i)} as u16;
                let second = unsafe {*bytes.get_unchecked(i+1)} as u16;
        
                let num = if second > SPACE {
                    i += 2;
                    first * 10 + second - 11 * ZERO
                } else {
                    i += 1;
                    first - ZERO
                };
                // if prev =< num, this will be huge and not pass the next test
                let diff = prev.wrapping_sub(num+1);
                if diff > 2 {
                    // failure: go to next line.
                    while i < bytes.len() && bytes[i] as u16 != NL {
                        i += 1;
                    }
                    i += 1;
                    break;
                }
                // i will be pointing at a space, NL, or EOF. if NL or EOF, exit this loop, and also, gg the line was fine!
                if i >= bytes.len() || bytes[i] as u16 == NL {
                    sum += 1;
                    i += 1;
                    break;
                }
                i += 1;
                prev = num;
            }
        }
    }
    sum
}

// roughly 170ms on my system (10000 runs)
pub fn part1_attempt(whole: &str) -> i64 {
    const SPACE: u8 = b' ';
    const NL: u8 = b'\n';
    const ZERO: u8 = b'0';
    // assert!(SPACE > NL);
    // assert!(b'0' > SPACE);
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
                if direction == 2 {
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
    let lines = get_lines(whole);
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
            } else if (prev - n == 0) || (prev - n > 3) {
                ok = false;
            }
            prev = n;
        }
        if ok {
            sum += 1;
        }
    }
    sum
}


pub fn part2(whole: &str) -> i64 {
    // If for a given line, you take a vector that has the difference between consecutive numbers.
    // That vector needs to follow the constraint that all values have a magnitude between 1 and 3, and,
    // all values have the same sign. A value with the wrong sign or wrong magnitude can be removed and fused
    // with the previous or next value (next_value += bad_value). bad values on the very edge can be removed outright.
    // this can only be done once, and after that the constraint must pass...
    // is that really a good way to solve it tho? hmmmm


    const SPACE: u16 = b' ' as u16;
    const NL: u16 = b'\n' as u16;
    const ZERO: u16 = b'0' as u16;
    let mut sum = 0;
    let bytes = whole.as_bytes();
    let mut i = 0;
    // each iteration is one LINE
    let mut nums = [0; 12];
    let mut diffs = [0; 11];
    while i < bytes.len() {
        // Assumption: each line has AT LEAST *FOUR* numbers. I sure hope that's true.

        // Actually, for now let's just collect the numbers to get a fine answer quickly.
        let mut j = 0;
        while i < bytes.len() {
            let first = unsafe {*bytes.get_unchecked(i)} as u16;
            let second = unsafe {*bytes.get_unchecked(i+1)} as u16;
            if second > SPACE {
                let num = first * 10 + second - 11 * ZERO;
                nums[j] = num;
                j += 1;
                i += 2;
                if i >= bytes.len() || bytes[i] as u16 == NL {
                    i += 1;
                    break;
                }
                i += 1;
            } else {
                let num = first - ZERO;
                nums[j] = num;
                j += 1;
                i += 2;
                if second == NL {
                    break;
                }
            };
        }

        // Now the actual logic... fine, I'll analyze the differences
        let amount = j;
        let diff_amount = amount - 1;
        let mut tendency = 0;
        
        for j in 0..diff_amount {
            tendency += (nums[j+1] as i16 - nums[j] as i16).signum() as i8;
            diffs[j] = nums[j+1] as i16 - nums[j] as i16; 
        }
        let mut mistakes = 0;
        let mut j = 0;
        if tendency > 0 {
            // should all be positive
            while j < diff_amount {
                if diffs[j] > 3 || diffs[j] <= 0 {
                    mistakes += 1;
                    if mistakes == 2 {
                        break;
                    }
                    // fuse left or right? whichever one saves an adjacent diff
                    // left diff will have saved itself, so we only care about helping right
                    // so, see if fusing with right is OK, otherwise go left
                    if j == diff_amount - 1 {
                    } else {
                        // ok, first try fusing right
                        let right_fused_diff = diffs[j+1] + diffs[j];
                        if right_fused_diff > 3 || right_fused_diff <= 0 {
                            // RIP. what about left?
                            if j != 0 {
                                let left_fused_diff = diffs[j-1] + diffs[j];
                                if left_fused_diff > 3 || left_fused_diff <= 0 {
                                    // RIP.
                                    mistakes += 1;
                                    break;
                                }
                            }
                        } else {
                            // ok, fuse with right, skip it too.
                            j += 2;
                            continue;
                        }
                    }
                }
                j += 1;
            }
        } else {
            // should all be negative
            while j < diff_amount {
                if diffs[j] < -3 || diffs[j] >= 0 {
                    mistakes += 1;
                    if mistakes == 2 {
                        break;
                    }
                    // fuse left or right? whichever one saves an adjacent diff
                    // left diff will have saved itself, so we only care about helping right
                    // so, see if fusing with right is OK, otherwise go left
                    if j == diff_amount - 1 {
                    } else {
                        // ok, first try fusing right
                        let right_fused_diff = diffs[j+1] + diffs[j];
                        if right_fused_diff < -3 || right_fused_diff >= 0 {
                            // RIP. what about left?
                            if j != 0 {
                                let left_fused_diff = diffs[j-1] + diffs[j];
                                if left_fused_diff < -3 || left_fused_diff >= 0 {
                                    // RIP.
                                    mistakes += 1;
                                    break;
                                }
                            }
                        } else {
                            // ok, fuse with right, skip it too.
                            j += 2;
                            continue;
                        }
                    }
                }
                j += 1;
            }
        }
        if mistakes <= 1 {
            sum += 1;
        }
    }
    sum
}