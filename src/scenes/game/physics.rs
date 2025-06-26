use agb::fixnum::{num, vec2};

use crate::scenes::game::{ball, peg};
use crate::scenes::game::{ball::Ball, peg::Peg};
use crate::types::{Fixed, Force};

fn gravity() -> Force {
    vec2(num!(0), num!(200))
}

pub fn move_and_collide(ball: &mut Ball, pegs: &mut [Peg], delta_time: Fixed) {
    let initial_position = ball.position;
    let initial_velocity = ball.velocity;

    ball.velocity = initial_velocity + gravity() * delta_time;
    ball.position = initial_position + ball.velocity * delta_time;

    // Wall collisions
    let radius = ball::radius();
    if ball.position.x < radius {
        ball.position.x = radius;
        ball.velocity.x = -ball.velocity.x;
    } else if ball.position.x > num!(160.0) - radius {
        ball.position.x = num!(160.0) - radius;
        ball.velocity.x = -ball.velocity.x;
    }

    // Freeze ball if it goes below screen (180 pixels to be completely hidden)
    if ball.position.y > num!(180.0) {
        ball.velocity = Force::new(num!(0.0), num!(0.0));
        return;
    }

    for peg in pegs.iter_mut() {
        let distance_vector = ball.position - peg.position;
        let distance = distance_vector.magnitude();
        let collision_distance = ball::radius() + peg::radius();

        if distance < collision_distance && distance > num!(0) {
            peg.touch();
            let normal = distance_vector / distance;
            ball.velocity -= normal * (ball.velocity.dot(normal) * num!(2));
            let overlap = collision_distance - distance;
            ball.position += normal * overlap;
        }
    }
}
