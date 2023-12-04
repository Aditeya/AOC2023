#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

impl Game {
    pub fn min_set(&self) -> Set {
        self.sets.iter().fold(Set::default(), |mut acc, set| {
            acc.r = acc.r.max(set.r);
            acc.g = acc.g.max(set.g);
            acc.b = acc.b.max(set.b);
            acc
        })
    }
}

impl From<(u32, Vec<Set>)> for Game {
    fn from(value: (u32, Vec<Set>)) -> Self {
        Self { id: value.0, sets: value.1 }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Set {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Set {
    pub fn is_round_valid(&self, rnd: &Set) -> bool {
        self.r <= rnd.r && self.g <= rnd.g && self.b <= rnd.b
    }

    pub fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

impl From<(u32, u32, u32)> for Set {
    fn from(value: (u32, u32, u32)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

