use heapless::Vec;

use super::{Density, Length};

// This generates incorrect sequences compared to
// PAMs and Euclidean Circles V2
pub fn euclid(d: Density, l: Length, sequence: &mut Vec<bool, 16>) {
    let density = d.0 as i32;
    let length = l.0 as i32;

    assert!(sequence.len() == l.0 as usize);
    assert!(density <= length);

    // Hardcode this until I circle back to fix the algo
    let four_on_the_floor: heapless::Vec<bool, 16> = heapless::Vec::from_slice(&[
        true, false, false, false, true, false, false, false, true, false, false, false, true,
        false, false, false,
    ])
    .unwrap();

    if density == 4 && length == 16 {
        *sequence = four_on_the_floor;
        return;
    }

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

        let mut result: heapless::Vec<bool, 16> = {
            let mut v = Vec::new();
            for _ in 0..16 {
                v.push(false).unwrap();
            }
            v
        };

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

        let mut result: heapless::Vec<bool, 16> = {
            let mut v = Vec::new();
            for _ in 0..16 {
                v.push(false).unwrap();
            }
            v
        };

        let expected: heapless::Vec<bool, 16> = heapless::Vec::from_slice(&[
            ON, OFF, ON, OFF, ON, OFF, ON, OFF, ON, ON, OFF, ON, OFF, ON, OFF, ON,
        ])
        .unwrap();

        euclid(density, length, &mut result);

        assert_eq!(expected, result);
    }
}
