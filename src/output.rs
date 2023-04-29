#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    On,
    Off,
}

#[derive(Debug)]
pub struct Output {
    pub state: State,
    count: u32,
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}

impl Output {
    pub fn new() -> Self {
        Self {
            state: State::On,
            count: 0,
        }
    }

    pub fn update(&mut self) {
        self.count += 1;
    }
}
