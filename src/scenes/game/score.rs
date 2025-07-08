pub type Component = i32;
pub type Damage = i32;
pub type Coins = i32;

#[derive(Clone, Copy, Debug)]
pub struct Score {
    base: Component,
    mult: Component,
    coins: Coins,
}

impl Score {
    pub fn new(base: Component, mult: Component, coins: Coins) -> Self {
        Self { base, mult, coins }
    }

    pub fn add(self, base: Component, mult: Component, coins: Coins) -> Self {
        Self {
            base: self.base + base,
            mult: self.mult + mult,
            coins: self.coins + coins,
        }
    }

    pub fn mult(self, base: Component, mult: Component, coins: Coins) -> Self {
        Self {
            base: self.base * base,
            mult: self.mult * mult,
            coins: self.coins * coins,
        }
    }

    pub fn apply(self, score: Score) -> Self {
        Self {
            base: self.base + score.base,
            mult: self.mult + score.mult,
            coins: self.coins + score.coins,
        }
    }

    pub fn extract(self) -> (Damage, Coins) {
        (self.base * self.mult, self.coins)
    }
}
