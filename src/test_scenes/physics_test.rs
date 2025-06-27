use crate::error::Error;
use crate::scenes::game::{ball::Ball, peg::Pegs, physics};
use crate::test_scenes::{TestResult, TestScene};
use crate::types::Fixed;
use agb::display::GraphicsFrame;
use agb::fixnum::{num, vec2};
use agb::input::{Button, ButtonController};
use alloc::vec;

pub struct PhysicsTestConfig {
    pub max_simulation_time: Fixed,
    pub description: &'static str,
}

impl PhysicsTestConfig {
    /// Simple test: ball falls through sparse pegs
    pub fn sparse_pegs() -> (Self, Ball, Pegs) {
        let config = Self {
            max_simulation_time: num!(5), // 5 seconds at 60fps = 300 frames
            description: "Ball falling through sparse pegs",
        };

        let ball = Ball::new(vec2(num!(61), num!(10)));

        let mut pegs = Pegs::new();
        // Add a few sparse pegs for the ball to interact with
        let _ = pegs.add_peg(vec2(num!(60), num!(40)), num!(15.0));
        let _ = pegs.add_peg(vec2(num!(100), num!(70)), num!(12.0));
        let _ = pegs.add_peg(vec2(num!(40), num!(100)), num!(18.0));
        let _ = pegs.add_peg(vec2(num!(120), num!(120)), num!(10.0));

        (config, ball, pegs)
    }

    /// Dense peg cluster test
    pub fn dense_cluster() -> (Self, Ball, Pegs) {
        let config = Self {
            max_simulation_time: num!(8),
            description: "Ball bouncing through dense peg cluster",
        };

        let mut pegs = Pegs::new();
        // Create a dense cluster of pegs in the middle area
        for x in (50..130).step_by(20) {
            for y in (50..140).step_by(18) {
                let force_radius = if (x + y) % 40 == 0 {
                    num!(20.0)
                } else {
                    num!(12.0)
                };
                let _ = pegs
                    .add_peg(vec2(Fixed::new(x), Fixed::new(y)), force_radius);
            }
        }

        let mut ball = Ball::new(vec2(num!(30), num!(10)));
        ball.velocity = vec2(num!(15), num!(5)); // Diagonal entry into cluster

        (config, ball, pegs)
    }

    /// Wall collision test
    pub fn wall_bounce() -> (Self, Ball, Pegs) {
        let config = Self {
            max_simulation_time: num!(6),
            description: "Ball bouncing off walls and pegs",
        };

        let mut pegs = Pegs::new();
        // Create a strategic peg arrangement for interesting wall bounces
        let _ = pegs.add_peg(vec2(num!(30), num!(50)), num!(15.0));
        let _ = pegs.add_peg(vec2(num!(130), num!(60)), num!(15.0));
        let _ = pegs.add_peg(vec2(num!(80), num!(90)), num!(18.0));
        let _ = pegs.add_peg(vec2(num!(50), num!(120)), num!(12.0));
        let _ = pegs.add_peg(vec2(num!(110), num!(120)), num!(12.0));

        let mut ball = Ball::new(vec2(num!(80), num!(5)));
        ball.velocity = vec2(num!(25), num!(10)); // Fast angled entry

        (config, ball, pegs)
    }
}

pub struct PhysicsTest {
    ball: Ball,
    pegs: Pegs,
    physics: physics::PhysicsState<physics::Grid>,
    config: PhysicsTestConfig,
    simulation_time: Fixed,
    frame_count: u32,
}

impl TestScene for PhysicsTest {
    fn new() -> Self {
        let (config, ball, pegs) = PhysicsTestConfig::sparse_pegs();
        Self::with_config(config, ball, pegs)
    }

    fn update(
        &mut self,
        input: &mut ButtonController,
    ) -> Result<TestResult, Error> {
        // Allow switching test scenarios with buttons
        if input.is_just_pressed(Button::L) {
            let (config, ball, pegs) = PhysicsTestConfig::sparse_pegs();
            *self = Self::with_config(config, ball, pegs);
            return Ok(TestResult::Running);
        }
        if input.is_just_pressed(Button::R) {
            let (config, ball, pegs) = PhysicsTestConfig::dense_cluster();
            *self = Self::with_config(config, ball, pegs);
            return Ok(TestResult::Running);
        }
        if input.is_just_pressed(Button::SELECT) {
            let (config, ball, pegs) = PhysicsTestConfig::wall_bounce();
            *self = Self::with_config(config, ball, pegs);
            return Ok(TestResult::Running);
        }

        // Exit test with START button
        if input.is_just_pressed(Button::START) {
            return Ok(TestResult::ManualCheck(
                "Test completed - verify physics behavior",
            ));
        }

        // Run physics simulation
        let delta_time = num!(1) / num!(60); // 60 FPS
        physics::update_ball_physics(
            &mut self.ball,
            &mut self.pegs,
            delta_time,
            &self.physics,
        );

        self.simulation_time += delta_time;
        self.frame_count += 1;

        // Check if simulation has run long enough
        if self.simulation_time >= self.config.max_simulation_time {
            return Ok(TestResult::ManualCheck(
                "Simulation completed - verify physics behavior",
            ));
        }

        // Check if ball has fallen off screen (test completed)
        if self.ball.position.y > num!(180) {
            return Ok(TestResult::ManualCheck(
                "Ball fell off screen - verify trajectory was correct",
            ));
        }

        Ok(TestResult::Running)
    }

    fn render(&mut self, frame: &mut GraphicsFrame) {
        // Render ball
        self.ball.show(frame);

        // Render pegs
        for i in 0..self.pegs.count {
            if self.pegs.present[i] {
                self.pegs.show(i, frame);
            }
        }
    }

    fn description(&self) -> &'static str {
        self.config.description
    }
}

impl PhysicsTest {
    pub fn with_config(
        config: PhysicsTestConfig,
        ball: Ball,
        pegs: Pegs,
    ) -> Self {
        let physics = physics::new(&pegs);

        Self {
            ball,
            pegs,
            physics,
            config,
            simulation_time: num!(0),
            frame_count: 0,
        }
    }

    pub fn current_stats(&self) -> (u32, Fixed, usize) {
        (
            self.frame_count,
            self.simulation_time,
            self.pegs.count - self.pegs.touched.iter().filter(|&&t| t).count(),
        )
    }
}
