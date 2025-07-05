pub struct Score {
    pub base: i32,
    pub mult: i32,

    total: i32,
}

impl Score {
    pub fn new() -> Self {
        Self {
            base: 0,
            mult: 1,
            total: 0,
        }
    }

    pub fn commit(&mut self) {
        self.total += self.base * self.mult;
        agb::println!(
            "{} * {} = {} += {}",
            self.mult,
            self.base,
            self.base * self.mult,
            self.total
        );
        self.base = 0;
        self.mult = 1;
    }

    pub fn total(&self) -> i32 {
        self.total
    }
}
