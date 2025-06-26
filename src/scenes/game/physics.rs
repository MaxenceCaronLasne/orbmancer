use agb::fixnum::{num, vec2};

use crate::scenes::game::{ball, peg};
use crate::scenes::game::{ball::Ball, peg::Peg};
use crate::types::{Fixed, Force};

// Physics constants
const GRAVITY_Y: f32 = 200.0;
const LEFT_WALL: f32 = 0.0;
const RIGHT_WALL: f32 = 160.0;
const SCREEN_BOTTOM: f32 = 180.0;
const WALL_BOUNCE_DAMPING: f32 = 0.9;
const PEG_BOUNCE_DAMPING: f32 = 0.9;
const ZERO: f32 = 0.0;

fn handle_wall_collisions(ball: &mut Ball) {
    let ball_radius = num!(ball::RADIUS);

    if ball.position.x < ball_radius {
        ball.position.x = ball_radius;
        ball.velocity.x = -ball.velocity.x * num!(WALL_BOUNCE_DAMPING);
    } else if ball.position.x > num!(RIGHT_WALL) - ball_radius {
        ball.position.x = num!(RIGHT_WALL) - ball_radius;
        ball.velocity.x = -ball.velocity.x * num!(WALL_BOUNCE_DAMPING);
    }
}

fn handle_peg_collisions(ball: &mut Ball, pegs: &mut [Peg]) {
    let ball_radius = num!(ball::RADIUS);
    let peg_radius = num!(peg::RADIUS);

    for peg in pegs.iter_mut() {
        let distance_vector = ball.position - peg.position;
        let distance = distance_vector.magnitude();
        let collision_distance = ball_radius + peg_radius;

        if distance < collision_distance && distance > num!(ZERO) {
            peg.touch();
            let normal = distance_vector / distance;
            let velocity_along_normal = ball.velocity.dot(normal);

            ball.velocity -= normal * (velocity_along_normal * num!(2.0));
            ball.velocity *= num!(PEG_BOUNCE_DAMPING);

            let overlap = collision_distance - distance;
            ball.position += normal * overlap;
        }
    }
}

pub fn move_and_collide(ball: &mut Ball, pegs: &mut [Peg], delta_time: Fixed) {
    let initial_position = ball.position;
    let initial_velocity = ball.velocity;

    ball.velocity =
        initial_velocity + vec2(num!(0), num!(GRAVITY_Y)) * delta_time;
    ball.position = initial_position + ball.velocity * delta_time;

    handle_wall_collisions(ball);

    if ball.position.y > num!(SCREEN_BOTTOM) {
        ball.velocity = Force::new(num!(ZERO), num!(ZERO));
        return;
    }

    handle_peg_collisions(ball, pegs);
}
