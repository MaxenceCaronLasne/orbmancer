pub struct Score {
    base: i32,
    mult: i32,

    total: i32,
}

impl Score {
    pub fn new() -> Self {
        Self {
            base: 0,
            mult: 0,
            total: 0,
        }
    }
    pub fn add_base(&mut self, value: i32) {
        self.base += value;
    }

    pub fn add_mult(&mut self, value: i32) {
        self.mult += value;
    }

    pub fn commit(&mut self) {
        self.total += self.base * self.mult;
        self.base = 0;
        self.mult = 0;
    }
}
