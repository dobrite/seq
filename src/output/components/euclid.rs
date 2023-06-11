use heapless::Vec;

use super::{Density, Length};

pub const MAX_STEPS: usize = 16;

pub type Sequence = Vec<bool, MAX_STEPS>;

// https://github.com/brianhouse/bjorklund (MIT)
pub fn euclid(d: Density, l: Length, sequence: &mut Sequence) {
    sequence.resize_default(l.0 as usize).unwrap();
    if d.0 == 0 {
        *sequence = sequence.iter_mut().map(|_| false).collect();
        return
    }

    let density = d.0 as i32;
    let length = l.0 as i32;

    assert!(sequence.len() == l.0 as usize);
    assert!(density <= length);

    let mut pattern = Vec::<bool, MAX_STEPS>::new();
    let mut counts = Vec::<i32, MAX_STEPS>::new();
    let mut remainders = Vec::<i32, MAX_STEPS>::new();
    let mut divisor = length - density;
    remainders.push(density).ok();
    let mut level = 0;

    loop {
        counts.push(divisor / remainders[level]).ok();
        remainders.push(divisor % remainders[level]).ok();
        divisor = remainders[level];
        level += 1;
        if remainders[level] <= 1 {
            break
        }
    }

    counts.push(divisor).ok();

    build(level as i32, &counts, &remainders, &mut pattern);
    let i = pattern.iter().position(|&elem| elem).unwrap_or(0);
    pattern.rotate_left(i);

    *sequence = pattern;
}

// This generates incorrect sequences compared to
// PAMs and Euclidean Circles V2
#[allow(dead_code)]
fn euclid_wrong(d: Density, l: Length, sequence: &mut Sequence) {
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

fn build(
    level: i32,
    counts: &Vec<i32, MAX_STEPS>,
    remainders: &Vec<i32, MAX_STEPS>,
    pattern: &mut Sequence,
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

        let expected: Sequence = Vec::from_slice(&[
            ON, OFF, OFF, OFF, ON, OFF, OFF, OFF, ON, OFF, OFF, OFF, ON, OFF, OFF, OFF,
        ])
        .unwrap();

        let mut result = Vec::new();
        euclid(density, length, &mut result);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_builds_a_sequence_of_length_sixteen_at_density_nine() {
        let density = Density(9);
        let length = Length(16);

        let expected: Sequence = Vec::from_slice(&[
            ON, OFF, ON, OFF, ON, OFF, ON, ON, OFF, ON, OFF, ON, OFF, ON, ON, OFF,
        ])
        .unwrap();

        let mut result = Vec::new();
        euclid(density, length, &mut result);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_builds_a_sequence_of_length_ten_at_density_four() {
        let density = Density(4);
        let length = Length(10);

        let expected: Sequence =
            Vec::from_slice(&[ON, OFF, ON, OFF, OFF, ON, OFF, ON, OFF, OFF]).unwrap();

        let mut result = Vec::new();
        euclid(density, length, &mut result);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_builds_a_sequence_of_length_two_at_density_zero() {
        let density = Density(0);
        let length = Length(2);

        let expected: Sequence = Vec::from_slice(&[OFF, OFF]).unwrap();

        let mut result = Vec::new();
        euclid(density, length, &mut result);

        assert_eq!(expected, result);
    }

    #[test]
    fn it_builds_a_sequence_of_length_two_at_density_zero_true_false() {
        let density = Density(0);
        let length = Length(2);

        let expected: Sequence = Vec::from_slice(&[OFF, OFF]).unwrap();

        let mut result = Vec::new();
        result.push(true).ok();
        result.push(false).ok();
        euclid(density, length, &mut result);

        assert_eq!(expected, result);
    }
}
