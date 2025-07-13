use crate::{Coordinates, Fixed, Force};
use agb::display::GraphicsFrame;
use agb::display::object::{Object, Sprite};
use agb::fixnum::{num, vec2};
use agb::include_aseprite;
use agb::rng::RandomNumberGenerator;

pub const RADIUS: i32 = 3;
pub const FORCE_RADII: [f32; 4] = [5.0, 10.0, 15.0, 20.0];

include_aseprite!(
    mod sprites,
    "assets/peg.aseprite"
);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Blue,
    Red,
    Yellow,
    Green,
}

fn sprite_from_kind(kind: Kind) -> &'static Sprite {
    match kind {
        Kind::Red => sprites::RED.sprite(0),
        Kind::Blue => sprites::BLUE.sprite(0),
        Kind::Yellow => sprites::YELLOW.sprite(0),
        Kind::Green => sprites::GREEN.sprite(0),
    }
}

pub struct Pegs<const N: usize> {
    pub positions: [Coordinates; N],
    pub velocities: [Force; N],
    pub force_radius_squared: [Fixed; N],
    pub sprite: [Object; N],
    pub showable: [bool; N],
    pub collidable: [bool; N],
    pub kind: [Kind; N],
}

impl<const N: usize> Pegs<N> {
    pub fn new(
        positions: [Coordinates; N],
        force_radius_squared: [Fixed; N],
        showable: [bool; N],
        collidable: [bool; N],
        kind: [Kind; N],
    ) -> Self {
        let sprite =
            core::array::from_fn(|i| Object::new(sprite_from_kind(kind[i])));

        Self {
            positions,
            velocities: [vec2(num!(0.0), num!(0.0)); N],
            force_radius_squared,
            sprite,
            showable,
            collidable,
            kind,
        }
    }

    pub fn spawn_pegs<const WALL_LEFT: i32, const WALL_RIGHT: i32>(
        rng: &mut RandomNumberGenerator,
    ) -> Pegs<N> {
        let peg_count = 50;
        let screen_height = 120;
        let min_y = 30;

        let mut positions = [vec2(num!(0), num!(0)); N];
        let mut force_radius_squared = [num!(20); N];
        let mut showable = [false; N];
        let mut collidable = [false; N];
        let mut kind = [Kind::Blue; N];

        for i in 0..peg_count {
            let x =
                WALL_LEFT + (rng.next_i32().abs() % (WALL_RIGHT - WALL_LEFT));
            let y = min_y + (rng.next_i32().abs() % (screen_height - min_y));

            let force_radius_index =
                (rng.next_i32().abs() % FORCE_RADII.len() as i32) as usize;
            let force_radius =
                Fixed::new(FORCE_RADII[force_radius_index] as i32);

            positions[i] = vec2(Fixed::new(x), Fixed::new(y));
            force_radius_squared[i] = force_radius * force_radius;
            showable[i] = true;
            collidable[i] = true;

            kind[i] = if i == 0 {
                Kind::Green
            } else if rng.next_i32() > 0 {
                Kind::Blue
            } else if rng.next_i32() > 0 {
                Kind::Yellow
            } else {
                Kind::Red
            }
        }

        Pegs::new(positions, force_radius_squared, showable, collidable, kind)
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        for i in 0..N {
            if self.showable[i] {
                self.sprite[i].set_pos(self.positions[i].round());
                self.sprite[i].show(frame);
            }
        }
    }
}
