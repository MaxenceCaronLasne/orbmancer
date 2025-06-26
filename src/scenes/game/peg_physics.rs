use crate::scenes::game::peg::Peg;
use crate::types::{Coordinate, Fixed, Force};
use agb::fixnum::{num, vec2};

// Physics constants
const INTERACTION_FORCE_STRENGTH: f32 = 50.0;
const MIN_INTERACTION_DISTANCE_SQUARED: f32 = 1.0; // 1.0^2  
const MAX_INTERACTION_DISTANCE_SQUARED: f32 = 3600.0; // 60.0^2
const ZERO_FORCE: f32 = 0.0;

// Movement boundaries
const PEG_MOVEMENT_LEFT_BOUND: f32 = 10.0;
const PEG_MOVEMENT_RIGHT_BOUND: f32 = 150.0;
const PEG_MOVEMENT_TOP_BOUND: f32 = 20.0;
const PEG_MOVEMENT_BOTTOM_BOUND: f32 = 140.0;

// Pre-allocated force accumulation buffer for performance
static mut PEG_FORCE_BUFFER: [Force; 50] = [Force {
    x: Fixed::from_raw(0),
    y: Fixed::from_raw(0),
}; 50];

pub fn update_peg_forces(pegs: &mut [Peg], delta_time: Fixed) {
    let n = pegs.len();
    if n < 2 || n > 50 {
        return;
    }

    // Use pre-allocated static buffer for performance
    unsafe {
        // Reset force accumulation buffer
        for i in 0..n {
            PEG_FORCE_BUFFER[i] = vec2(num!(ZERO_FORCE), num!(ZERO_FORCE));
        }

        for i in 0..n {
            for j in (i + 1)..n {
                if pegs[i].is_touched() || pegs[j].is_touched() {
                    continue;
                }

                let distance_vector = pegs[j].position - pegs[i].position;
                let distance_squared = distance_vector.magnitude_squared();

                if distance_squared < num!(MIN_INTERACTION_DISTANCE_SQUARED)
                    || distance_squared > num!(MAX_INTERACTION_DISTANCE_SQUARED)
                {
                    continue;
                }

                // Calculate interaction forces for both pegs
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

                PEG_FORCE_BUFFER[i] += force_on_peg_i;
                PEG_FORCE_BUFFER[j] += force_on_peg_j;
            }
        }

        // Apply accumulated forces to peg positions
        for (i, peg) in pegs.iter_mut().enumerate() {
            if peg.is_touched() {
                continue;
            }

            let velocity_change: Force = PEG_FORCE_BUFFER[i] * delta_time;
            peg.position += velocity_change;

            // Keep pegs within movement boundaries
            peg.position.x = peg.position.x.clamp(
                num!(PEG_MOVEMENT_LEFT_BOUND),
                num!(PEG_MOVEMENT_RIGHT_BOUND),
            );
            peg.position.y = peg.position.y.clamp(
                num!(PEG_MOVEMENT_TOP_BOUND),
                num!(PEG_MOVEMENT_BOTTOM_BOUND),
            );
        }
    } // End unsafe block
}

fn calculate_interaction_force(
    distance_squared: Fixed,
    force_radius_squared: Fixed,
    direction_vector: Coordinate,
) -> Force {
    // Optimized force calculation using squared distances only
    if distance_squared < force_radius_squared {
        // Repulsion: push pegs apart when closer than force radius
        direction_vector * num!(INTERACTION_FORCE_STRENGTH) * num!(0.01) // Scaled for unnormalized vector
    } else {
        // Attraction: pull pegs together when farther than force radius
        -direction_vector * num!(INTERACTION_FORCE_STRENGTH) * num!(0.003) // Weaker attraction force
    }
}

