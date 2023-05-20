use heapless::Vec;

use super::components::Rate;

mod sequence;

#[derive(Debug, PartialEq)]
pub struct Euclid {
    cycle_target: u32,
    density: u32,
    length: u32,
    pub(crate) edge_change: bool,
    pub(crate) on: bool,
    rate: Rate,
    resolution: u32,
    sequence: Vec<bool, 16>,
}

impl Default for Euclid {
    fn default() -> Self {
        Self::new(1_920, Rate::Unity, 16, 4)
    }
}

impl Euclid {
    pub fn new(resolution: u32, rate: Rate, length: u32, density: u32) -> Self {
        let mut sequence: Vec<bool, 16> = Vec::new();
        for _ in 0..16 {
            sequence.push(false).unwrap();
        }
        sequence::fill(density, length, &mut sequence);

        Self {
            cycle_target: 1_920,
            density,
            edge_change: false,
            length,
            on: true,
            rate,
            resolution,
            sequence,
        }
    }

    #[allow(dead_code)]
    pub fn tick(&mut self, count: u32) {
        let initial_on = self.on;

        if self.on {
            self.on = false; // TODO: this is only on for one tick
        }

        if count % self.cycle_target == 0 {
            let index = count / self.cycle_target % self.length;
            self.on = self.sequence[index as usize];
        }

        self.edge_change = initial_on != self.on;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ON: bool = true;
    const OFF: bool = false;

    #[test]
    fn it_new() {
        let rate = Rate::Unity;
        let length = 16;
        let density = 4;
        let euclid = Euclid::new(1_920, rate, length, density);
        let mut sequence: Vec<bool, 16> = Vec::new();
        for _ in 0..16 {
            sequence.push(false).unwrap();
        }
        sequence::fill(density, length, &mut sequence);

        let expected = Euclid {
            cycle_target: 1_920,
            density,
            edge_change: false,
            length,
            on: true,
            rate,
            resolution: 1_920,
            sequence,
        };

        assert_eq!(expected, euclid);
    }

    #[test]
    fn it_updates_on_at_length_sixteen_at_density_four() {
        let mut euclid = Euclid::new(1_920, Rate::Unity, 16, 4);

        euclid.tick(0);
        assert_eq!(ON, euclid.on);

        euclid.tick(1_920);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 2);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 3);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 4);
        assert_eq!(ON, euclid.on);

        euclid.tick(1_920 * 5);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 6);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 7);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 8);
        assert_eq!(ON, euclid.on);

        euclid.tick(1_920 * 9);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 10);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 11);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 12);
        assert_eq!(ON, euclid.on);

        euclid.tick(1_920 * 13);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 14);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 15);
        assert_eq!(OFF, euclid.on);

        euclid.tick(1_920 * 16);
        assert_eq!(ON, euclid.on);
    }

    #[test]
    fn it_updates_edge_change_at_length_sixteen_at_density_four() {
        let mut euclid = Euclid::new(1_920, Rate::Unity, 16, 4);

        assert_eq!(OFF, euclid.edge_change);
        euclid.tick(0);
        assert_eq!(OFF, euclid.edge_change);
        euclid.tick(1);
        assert_eq!(ON, euclid.edge_change);
        euclid.tick(2);
        assert_eq!(OFF, euclid.edge_change);

        euclid.tick(1_919);
        assert_eq!(OFF, euclid.edge_change);
        euclid.tick(1_920);
        assert_eq!(OFF, euclid.edge_change);
        euclid.tick(1_921);
        assert_eq!(OFF, euclid.edge_change);
    }
}

