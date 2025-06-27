use crate::error::Error;
use crate::scenes::game::{ball::Ball, peg::Pegs, physics};
use crate::test_scenes::{TestResult, TestScene};
use crate::types::Fixed;
use agb::display::GraphicsFrame;
use agb::fixnum::{num, vec2};
use agb::input::{Button, ButtonController};
use alloc::vec;

/// Benchmark configuration for different physics stress tests
pub struct PhysicsBenchConfig {
    pub max_simulation_time: Fixed,
    pub description: &'static str,
    pub peg_count: usize,
    pub ball_count: usize,
}

impl PhysicsBenchConfig {
    /// Lightweight benchmark: Few pegs, single ball
    pub fn lightweight() -> (Self, Vec<Ball>, Pegs) {
        let config = Self {
            max_simulation_time: num!(10), // 10 seconds
            description: "Lightweight: 5 pegs, 1 ball",
            peg_count: 5,
            ball_count: 1,
        };

        let balls = vec![Ball::new(vec2(num!(61), num!(10)))];

        let mut pegs = Pegs::new();
        // Strategic placement for interesting interactions
        let _ = pegs.add_peg(vec2(num!(40), num!(50)), num!(15.0));
        let _ = pegs.add_peg(vec2(num!(80), num!(70)), num!(12.0));
        let _ = pegs.add_peg(vec2(num!(120), num!(90)), num!(18.0));
        let _ = pegs.add_peg(vec2(num!(60), num!(110)), num!(10.0));
        let _ = pegs.add_peg(vec2(num!(100), num!(130)), num!(16.0));

        (config, balls, pegs)
    }

    /// Medium load benchmark: Moderate peg density
    pub fn medium_load() -> (Self, Vec<Ball>, Pegs) {
        let config = Self {
            max_simulation_time: num!(15),
            description: "Medium load: 15 pegs, 2 balls",
            peg_count: 15,
            ball_count: 2,
        };

        let balls = vec![
            Ball::new(vec2(num!(30), num!(10))),
            Ball::new(vec2(num!(90), num!(10))),
        ];

        let mut pegs = Pegs::new();
        // Create a grid pattern with some randomness
        for y in (40..140).step_by(25) {
            for x in (30..130).step_by(25) {
                let offset_x = if (x + y) % 50 == 0 { 8 } else { -8 };
                let offset_y = if (x + y) % 75 == 0 { 5 } else { -5 };
                let force_radius = if (x + y) % 100 == 0 { num!(20.0) } else { num!(12.0) };
                
                let _ = pegs.add_peg(
                    vec2(Fixed::new(x + offset_x), Fixed::new(y + offset_y)), 
                    force_radius
                );
                
                if pegs.count >= 15 { break; }
            }
            if pegs.count >= 15 { break; }
        }

        (config, balls, pegs)
    }

    /// Heavy load benchmark: Dense peg field, stress test
    pub fn heavy_load() -> (Self, Vec<Ball>, Pegs) {
        let config = Self {
            max_simulation_time: num!(20),
            description: "Heavy load: 25+ pegs, 3 balls",
            peg_count: 25,
            ball_count: 3,
        };

        let balls = vec![
            Ball::new(vec2(num!(20), num!(5))),
            Ball::new(vec2(num!(80), num!(8))),
            Ball::new(vec2(num!(120), num!(12))),
        ];

        let mut pegs = Pegs::new();
        // Dense grid pattern to stress test spatial optimization
        for y in (35..145).step_by(18) {
            for x in (25..135).step_by(18) {
                let force_radius = match (x + y) % 60 {
                    0..=20 => num!(25.0),  // Large pegs
                    21..=40 => num!(15.0), // Medium pegs
                    _ => num!(8.0),        // Small pegs
                };
                
                let _ = pegs.add_peg(vec2(Fixed::new(x), Fixed::new(y)), force_radius);
            }
        }

        (config, balls, pegs)
    }
}

pub struct PhysicsBench {
    balls: Vec<Ball>,
    pegs: Pegs,
    physics: physics::PhysicsState<physics::Grid>,
    config: PhysicsBenchConfig,
    simulation_time: Fixed,
    frame_count: u32,
    current_benchmark: usize,
    #[cfg(feature = "benchmark")]
    timers: Option<agb::timer::Timers>,
}

impl TestScene for PhysicsBench {
    fn new() -> Self {
        let (config, balls, pegs) = PhysicsBenchConfig::lightweight();
        Self::with_config(config, balls, pegs, 0)
    }

    fn update(
        &mut self,
        input: &mut ButtonController,
    ) -> Result<TestResult, Error> {
        // Cycle through benchmark scenarios
        if input.is_just_pressed(Button::L) {
            let (config, balls, pegs) = PhysicsBenchConfig::lightweight();
            *self = Self::with_config(config, balls, pegs, 0);
            #[cfg(feature = "benchmark")]
            crate::bench::reset_physics_stats();
            return Ok(TestResult::Running);
        }
        if input.is_just_pressed(Button::R) {
            let (config, balls, pegs) = PhysicsBenchConfig::medium_load();
            *self = Self::with_config(config, balls, pegs, 1);
            #[cfg(feature = "benchmark")]
            crate::bench::reset_physics_stats();
            return Ok(TestResult::Running);
        }
        if input.is_just_pressed(Button::SELECT) {
            let (config, balls, pegs) = PhysicsBenchConfig::heavy_load();
            *self = Self::with_config(config, balls, pegs, 2);
            #[cfg(feature = "benchmark")]
            crate::bench::reset_physics_stats();
            return Ok(TestResult::Running);
        }

        // Print benchmark stats
        #[cfg(feature = "benchmark")]
        if input.is_just_pressed(Button::A) {
            crate::bench::log_physics_breakdown();
            return Ok(TestResult::Running);
        }

        // Exit test
        if input.is_just_pressed(Button::START) {
            #[cfg(feature = "benchmark")]
            crate::bench::log_physics_breakdown();
            return Ok(TestResult::ManualCheck(
                "Physics benchmark completed - check logs for performance metrics",
            ));
        }

        // Run physics simulation with benchmarking
        let delta_time = num!(1) / num!(60); // 60 FPS

        #[cfg(feature = "benchmark")]
        for ball in &mut self.balls {
            physics::update_ball_physics_with_timers(
                ball,
                &mut self.pegs,
                delta_time,
                &self.physics,
                self.timers.as_ref(),
            );
        }

        #[cfg(not(feature = "benchmark"))]
        for ball in &mut self.balls {
            physics::update_ball_physics(
                ball,
                &mut self.pegs,
                delta_time,
                &self.physics,
            );
        }

        #[cfg(feature = "benchmark")]
        let _ = physics::update_peg_physics_with_grid_and_timers(
            &mut self.pegs,
            delta_time,
            &mut self.physics,
            self.timers.as_ref(),
        );

        #[cfg(not(feature = "benchmark"))]
        let _ = physics::update_peg_physics_with_grid(
            &mut self.pegs,
            delta_time,
            &mut self.physics,
        );

        self.simulation_time += delta_time;
        self.frame_count += 1;

        // Log periodic performance stats
        #[cfg(feature = "benchmark")]
        if self.frame_count % 300 == 0 { // Every 5 seconds at 60fps
            crate::bench::log_physics_breakdown();
        }

        // Check if simulation has run long enough
        if self.simulation_time >= self.config.max_simulation_time {
            #[cfg(feature = "benchmark")]
            crate::bench::log_physics_breakdown();
            return Ok(TestResult::ManualCheck(
                "Benchmark simulation completed - check performance metrics",
            ));
        }

        // Reset balls if they all fall off screen
        let all_off_screen = self.balls.iter().all(|ball| ball.position.y > num!(180));
        if all_off_screen {
            for (i, ball) in self.balls.iter_mut().enumerate() {
                ball.position = vec2(num!(20) + Fixed::new(i as i32 * 30), num!(5));
                ball.velocity = vec2(num!(0), num!(0));
            }
        }

        Ok(TestResult::Running)
    }

    fn render(&mut self, frame: &mut GraphicsFrame) {
        // Render all balls
        for ball in &mut self.balls {
            ball.show(frame);
        }

        // Render pegs
        for i in 0..self.pegs.count {
            if self.pegs.present[i] {
                self.pegs.show(i, frame);
            }
        }
    }

    fn description(&self) -> &'static str {
        match self.current_benchmark {
            0 => "Physics Benchmark: Lightweight (L=Light, R=Medium, Select=Heavy, A=Stats)",
            1 => "Physics Benchmark: Medium Load (L=Light, R=Medium, Select=Heavy, A=Stats)",  
            2 => "Physics Benchmark: Heavy Load (L=Light, R=Medium, Select=Heavy, A=Stats)",
            _ => "Physics Benchmark: Unknown",
        }
    }
}

impl PhysicsBench {
    pub fn with_config(
        config: PhysicsBenchConfig,
        balls: Vec<Ball>,
        pegs: Pegs,
        benchmark_type: usize,
    ) -> Self {
        let physics = physics::new(&pegs);

        Self {
            balls,
            pegs,
            physics,
            config,
            simulation_time: num!(0),
            frame_count: 0,
            current_benchmark: benchmark_type,
            #[cfg(feature = "benchmark")]
            timers: None, // Will be set when timers are available
        }
    }

    #[cfg(feature = "benchmark")]
    pub fn set_timers(&mut self, timers: agb::timer::Timers) {
        self.timers = Some(timers);
    }

    pub fn current_stats(&self) -> (u32, Fixed, usize, usize) {
        (
            self.frame_count,
            self.simulation_time,
            self.pegs.count,
            self.balls.len(),
        )
    }
}