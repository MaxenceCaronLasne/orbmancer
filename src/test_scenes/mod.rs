use crate::error::Error;
use agb::{display::GraphicsFrame, input::ButtonController};

mod physics_test;

pub use physics_test::PhysicsTest;

/// Trait for test scenes that can be run in isolation
pub trait TestScene {
    /// Initialize the test scene with specific configuration
    fn new() -> Self
    where
        Self: Sized;

    /// Update the scene for one frame
    fn update(&mut self, input: &mut ButtonController) -> Result<TestResult, Error>;

    /// Render the scene
    fn render(&mut self, frame: &mut GraphicsFrame);

    /// Get a description of what this test is checking
    fn description(&self) -> &'static str;
}

/// Result of a test scene update
#[derive(Debug, PartialEq)]
pub enum TestResult {
    /// Test is still running
    Running,
    /// Test completed successfully
    Passed,
    /// Test failed
    Failed(&'static str),
    /// Test completed but needs manual verification
    ManualCheck(&'static str),
}

/// Run a test scene for a specified duration or until completion
pub fn run_test_scene<T: TestScene>(
    gba: &mut agb::Gba,
    max_frames: u32,
) -> Result<TestResult, Error> {
    let mut scene = T::new();
    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();
    let mut frame_count = 0;

    loop {
        input.update();
        
        match scene.update(&mut input)? {
            TestResult::Running => {
                if frame_count >= max_frames {
                    return Ok(TestResult::ManualCheck("Test timed out - manual verification needed"));
                }
            }
            result => return Ok(result),
        }

        let mut frame = gfx.frame();
        scene.render(&mut frame);
        frame.commit();
        
        frame_count += 1;
    }
}