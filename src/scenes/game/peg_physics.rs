use crate::scenes::game::peg::Peg;
use crate::types::{Coordinate, Fixed, Force};
use agb::fixnum::{num, vec2};

const INTERACTION_FORCE_STRENGTH: f32 = 50.0;
const MIN_INTERACTION_DISTANCE_SQUARED: f32 = 0.0;
const MAX_INTERACTION_DISTANCE_SQUARED: f32 = 60.0 * 60.0;

const PEG_MOVEMENT_LEFT_BOUND: f32 = 10.0;
const PEG_MOVEMENT_RIGHT_BOUND: f32 = 150.0;
const PEG_MOVEMENT_TOP_BOUND: f32 = 20.0;
const PEG_MOVEMENT_BOTTOM_BOUND: f32 = 140.0;

const MAX_PEGS: usize = 50;

pub fn update_peg_forces(pegs: &mut [Peg], delta_time: Fixed) {
    let n = pegs.len();
    if !(2..=MAX_PEGS).contains(&n) {
        return;
    }

    let mut force_buffer = [vec2(num!(0), num!(0)); MAX_PEGS];

    for i in 0..n {
        for j in 0..n {
            if i == j || pegs[i].is_touched() || pegs[j].is_touched() {
                continue;
            }

            let distance_vector = pegs[j].position - pegs[i].position;
            let distance_squared = distance_vector.magnitude_squared();

            if distance_squared > num!(MAX_INTERACTION_DISTANCE_SQUARED) {
                continue;
            }

            let peg_i_radius_squared =
                pegs[i].force_radius * pegs[i].force_radius;
            let peg_j_radius_squared =
                pegs[j].force_radius * pegs[j].force_radius;

            let force_on_peg_i: Force = calculate_interaction_force(
                distance_squared,
                peg_i_radius_squared,
                distance_vector,
            );
            let force_on_peg_j: Force = calculate_interaction_force(
                distance_squared,
                peg_j_radius_squared,
                -distance_vector,
            );

            force_buffer[i] += force_on_peg_i;
            force_buffer[j] += force_on_peg_j;
        }
    }

    for (i, peg) in pegs.iter_mut().enumerate() {
        if peg.is_touched() {
            continue;
        }

        let velocity_change: Force = force_buffer[i] * delta_time;
        peg.position += velocity_change;

        peg.position.x = peg.position.x.clamp(
            num!(PEG_MOVEMENT_LEFT_BOUND),
            num!(PEG_MOVEMENT_RIGHT_BOUND),
        );
        peg.position.y = peg.position.y.clamp(
            num!(PEG_MOVEMENT_TOP_BOUND),
            num!(PEG_MOVEMENT_BOTTOM_BOUND),
        );
    }
}

fn calculate_interaction_force(
    distance_squared: Fixed,
    force_radius_squared: Fixed,
    direction_vector: Coordinate,
) -> Force {
    if distance_squared < force_radius_squared {
        -direction_vector * num!(INTERACTION_FORCE_STRENGTH) * num!(0.1)
    } else {
        direction_vector * num!(INTERACTION_FORCE_STRENGTH) * num!(0.01) * 1
            / distance_squared
    }
}
