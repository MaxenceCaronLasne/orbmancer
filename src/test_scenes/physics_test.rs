use crate::error::Error;
use crate::scenes::game::{ball::Ball, peg::Peg, physics};
use crate::test_scenes::{TestResult, TestScene};
use crate::types::Fixed;
use agb::display::GraphicsFrame;
use agb::fixnum::{num, vec2};
use agb::input::{Button, ButtonController};
use alloc::alloc::Global;
use alloc::vec;
use alloc::vec::Vec;

/// Configuration for physics test scenario
pub struct PhysicsTestConfig {
    pub ball_position: (Fixed, Fixed),
    pub ball_velocity: (Fixed, Fixed),
    pub peg_positions: Vec<(Fixed, Fixed)>,
    pub max_simulation_time: Fixed,
    pub description: &'static str,
}

impl PhysicsTestConfig {
    /// Simple test: ball falls through sparse pegs
    pub fn sparse_pegs() -> Self {
        Self {
            ball_position: (num!(81), num!(10)),
            ball_velocity: (num!(0), num!(0)),
            peg_positions: vec![
                (num!(40), num!(60)),
                (num!(120), num!(80)),
                (num!(80), num!(100)),
                (num!(60), num!(120)),
                (num!(100), num!(140)),
            ],
            max_simulation_time: num!(5), // 5 seconds at 60fps = 300 frames
            description: "Ball falling through sparse pegs",
        }
    }

    /// Dense peg cluster test
    pub fn dense_cluster() -> Self {
        let mut pegs = Vec::new();
        // Create a dense cluster of pegs
        for x in (40..120).step_by(15) {
            for y in (60..120).step_by(15) {
                pegs.push((Fixed::new(x), Fixed::new(y)));
            }
        }

        Self {
            ball_position: (num!(80), num!(10)),
            ball_velocity: (num!(10), num!(0)), // Small horizontal velocity
            peg_positions: pegs,
            max_simulation_time: num!(5),
            description: "Ball bouncing through dense peg cluster",
        }
    }

    /// Wall collision test
    pub fn wall_bounce() -> Self {
        Self {
            ball_position: (num!(10), num!(10)),
            ball_velocity: (num!(-20), num!(0)), // Fast leftward velocity
            peg_positions: vec![(num!(80), num!(60)), (num!(80), num!(100))],
            max_simulation_time: num!(3),
            description: "Ball bouncing off walls and pegs",
        }
    }
}

pub struct PhysicsTest {
    ball: Ball,
    pegs: Vec<Peg>,
    config: PhysicsTestConfig,
    simulation_time: Fixed,
    frame_count: u32,
}

impl TestScene for PhysicsTest {
    fn new() -> Self {
        Self::with_config(PhysicsTestConfig::sparse_pegs())
    }

    fn update(
        &mut self,
        input: &mut ButtonController,
    ) -> Result<TestResult, Error> {
        // Allow switching test scenarios with buttons
        if input.is_just_pressed(Button::L) {
            *self = Self::with_config(PhysicsTestConfig::sparse_pegs());
            return Ok(TestResult::Running);
        }
        if input.is_just_pressed(Button::R) {
            *self = Self::with_config(PhysicsTestConfig::dense_cluster());
            return Ok(TestResult::Running);
        }
        if input.is_just_pressed(Button::SELECT) {
            *self = Self::with_config(PhysicsTestConfig::wall_bounce());
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
        physics::move_and_collide(&mut self.ball, &mut self.pegs, delta_time);

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
        for peg in self.pegs.iter_mut() {
            peg.show(frame);
        }
    }

    fn description(&self) -> &'static str {
        self.config.description
    }
}

impl PhysicsTest {
    pub fn with_config(config: PhysicsTestConfig) -> Self {
        // Create ball at configured position
        let mut ball =
            Ball::new(vec2(config.ball_position.0, config.ball_position.1));
        ball.velocity = vec2(config.ball_velocity.0, config.ball_velocity.1);

        // Create pegs at configured positions
        let mut pegs = Vec::<Peg>::new_in(Global);
        for (x, y) in &config.peg_positions {
            pegs.push(Peg::new(vec2(*x, *y), num!(25.0))); // Use default force radius for tests
        }

        Self {
            ball,
            pegs,
            config,
            simulation_time: num!(0),
            frame_count: 0,
        }
    }

    pub fn current_stats(&self) -> (u32, Fixed, usize) {
        (
            self.frame_count,
            self.simulation_time,
            self.pegs.iter().filter(|p| p.is_touched()).count(),
        )
    }
}

