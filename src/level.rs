use crate::peg::Kind;

pub struct Level {
    target_score: i32,
    blue_qty: i32,
    red_qty: i32,
    yellow_qty: i32,
    green_qty: i32,
}

impl Level {
    pub fn new(
        target_score: i32,
        blue_qty: i32,
        red_qty: i32,
        yellow_qty: i32,
        green_qty: i32,
    ) -> Self {
        Self {
            target_score,
            blue_qty,
            red_qty,
            yellow_qty,
            green_qty,
        }
    }

    pub fn new_test_level() -> Self {
        Self {
            target_score: 100,
            blue_qty: 20,
            red_qty: 10,
            yellow_qty: 5,
            green_qty: 2,
        }
    }

    pub fn target_score(&self) -> i32 {
        self.target_score
    }

    pub fn peg_count<const N: usize>(&self) -> [Option<Kind>; N] {
        let mut result = [None; N];
        let mut idx = 0;

        for _ in 0..self.green_qty {
            if idx < N {
                result[idx] = Some(Kind::Green);
                idx += 1;
            }
        }
        for _ in 0..self.blue_qty {
            if idx < N {
                result[idx] = Some(Kind::Blue);
                idx += 1;
            }
        }
        for _ in 0..self.red_qty {
            if idx < N {
                result[idx] = Some(Kind::Red);
                idx += 1;
            }
        }
        for _ in 0..self.yellow_qty {
            if idx < N {
                result[idx] = Some(Kind::Yellow);
                idx += 1;
            }
        }
        result
    }
}
