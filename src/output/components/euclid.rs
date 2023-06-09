use heapless::Vec;

use super::{Density, Length};

// This generates incorrect sequences compared to
// PAMs and Euclidean Circles V2
#[allow(dead_code)]
fn euclid_wrong(d: Density, l: Length, sequence: &mut Vec<bool, 16>) {
    let density = d.0 as i32;
    let length = l.0 as i32;

    assert!(sequence.len() == l.0 as usize);
    assert!(density <= length);

    let mut error = (2 * density) - length;

    for elem in sequence.iter_mut() {
        if error > 0 {
            error -= 2 * length;
            error += 2 * density;

            *elem = true
        } else {
            error += 2 * density;

            *elem = false
        }
    }

    let i = sequence.iter().position(|&elem| elem).unwrap_or(0);
    sequence.rotate_left(i);
}

// https://github.com/brianhouse/bjorklund (MIT)
pub fn euclid(d: Density, l: Length, sequence: &mut Vec<bool, 16>) {
    let density = d.0 as i32;
    let length = l.0 as i32;

    assert!(sequence.len() == l.0 as usize);
    assert!(density <= length);

    let mut pattern = Vec::<bool, 16>::new();
    let mut counts = Vec::<i32, 16>::new();
    let mut remainders = Vec::<i32, 16>::new();
    let mut divisor = length - density;
    remainders.push(density).ok();
    let mut level = 0;

    loop {
        counts.push(divisor / remainders[level]).ok();
        remainders.push(divisor % remainders[level]).ok();
        divisor = remainders[level];
        level += 1;
        if remainders[level] <= 1 {
            break;
        }
    }

    counts.push(divisor).ok();

    build(level as i32, &counts, &remainders, &mut pattern);
    let i = pattern.iter().position(|&elem| elem).unwrap_or(0);
    pattern.rotate_left(i);

    *sequence = pattern;
}

fn build(
    level: i32,
    counts: &Vec<i32, 16>,
    remainders: &Vec<i32, 16>,
    pattern: &mut Vec<bool, 16>,
) {
    if level == -1 {
        pattern.push(false).ok();
    } else if level == -2 {
        pattern.push(true).ok();
    } else {
        for _ in 0..counts[level as usize] {
            build(level - 1, counts, remainders, pattern)
        }

        if remainders[level as usize] != 0 {
            build(level - 2, counts, remainders, pattern)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ON: bool = true;
    const OFF: bool = false;

    #[test]
    fn it_builds_a_sequence_of_length_sixteen_at_density_four() {
        let density = Density(4);
        let length = Length(16);

        let mut result = Vec::new();
        result.resize_default(16).ok();

        let expected: heapless::Vec<bool, 16> = heapless::Vec::from_slice(&[
            ON, OFF, OFF, OFF, ON, OFF, OFF, OFF, ON, OFF, OFF, OFF, ON, OFF, OFF, OFF,
        ])
        .unwrap();

        euclid(density, length, &mut result);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_builds_a_sequence_of_length_sixteen_at_density_nine() {
        let density = Density(9);
        let length = Length(16);

        let mut result = Vec::new();
        result.resize_default(16).ok();

        let expected: heapless::Vec<bool, 16> = heapless::Vec::from_slice(&[
            ON, OFF, ON, OFF, ON, OFF, ON, ON, OFF, ON, OFF, ON, OFF, ON, ON, OFF,
        ])
        .unwrap();

        euclid(density, length, &mut result);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_builds_a_sequence_of_length_ten_at_density_four() {
        let density = Density(4);
        let length = Length(10);

        let mut result = Vec::new();
        result.resize_default(10).ok();

        let expected: heapless::Vec<bool, 16> =
            heapless::Vec::from_slice(&[ON, OFF, ON, OFF, OFF, ON, OFF, ON, OFF, OFF]).unwrap();

        euclid(density, length, &mut result);

        assert_eq!(expected, result);
    }
}
