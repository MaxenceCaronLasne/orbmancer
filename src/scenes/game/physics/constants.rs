use crate::types::Fixed;
use agb::fixnum::num;

// Type aliases for clarity and type safety
/// Distance measurement in game units
pub type Distance = Fixed;
/// Time measurement in seconds  
pub type Time = Fixed;
/// Force measurement in game units per second squared
pub type ForceStrength = Fixed;
/// Damping coefficient (0.0 = no damping, 1.0 = no energy loss)
pub type DampingFactor = Fixed;

// Physics constants
pub const GRAVITY_Y: f32 = 200.0;
pub const ZERO: f32 = 0.0;

// Wall and screen boundaries
pub const LEFT_WALL: f32 = 0.0;
pub const RIGHT_WALL: f32 = 160.0;
pub const SCREEN_BOTTOM: f32 = 180.0;

// Damping coefficients
pub const WALL_BOUNCE_DAMPING: f32 = 0.9;
pub const PEG_BOUNCE_DAMPING: f32 = 0.9;

// Peg interaction forces
pub const INTERACTION_FORCE_STRENGTH: f32 = 200.0;
pub const MAX_INTERACTION_DISTANCE_SQUARED: f32 = 60.0 * 60.0;

// Peg movement boundaries
pub const PEG_MOVEMENT_LEFT_BOUND: f32 = 10.0;
pub const PEG_MOVEMENT_RIGHT_BOUND: f32 = 150.0;
pub const PEG_MOVEMENT_TOP_BOUND: f32 = 20.0;
pub const PEG_MOVEMENT_BOTTOM_BOUND: f32 = 140.0;

/// Physics configuration for tuning gameplay behavior
/// 
/// All physics parameters can be adjusted at runtime to create different gameplay feels.
/// Values are validated to ensure they remain within reasonable bounds.
/// 
/// # Examples
/// 
/// ```rust
/// use crate::scenes::game::physics::PhysicsConfig;
/// use agb::fixnum::num;
/// 
/// // Create low-gravity, bouncy physics
/// let mut config = PhysicsConfig::default();
/// config.gravity_y = num!(100.0);           // Half gravity
/// config.peg_bounce_damping = num!(0.95);   // Very bouncy
/// ```
#[derive(Clone, Copy, Debug)]
pub struct PhysicsConfig {
    /// Downward gravity acceleration (positive = down)
    /// 
    /// Typical range: 50.0 - 500.0
    /// Default: 200.0
    pub gravity_y: ForceStrength,
    
    /// Energy retention when ball hits walls (0.0 = all energy lost, 1.0 = no energy lost)
    /// 
    /// Range: 0.0 - 1.0
    /// Default: 0.9
    pub wall_bounce_damping: DampingFactor,
    
    /// Energy retention when ball hits pegs (0.0 = all energy lost, 1.0 = no energy lost)
    /// 
    /// Range: 0.0 - 1.0  
    /// Default: 0.9
    pub peg_bounce_damping: DampingFactor,
    
    /// Strength of peg-to-peg interaction forces
    /// 
    /// Typical range: 50.0 - 1000.0
    /// Default: 200.0
    pub interaction_force_strength: ForceStrength,
    
    /// Maximum distance (squared) for peg interactions
    /// 
    /// Pegs further apart than this distance don't interact.
    /// Default: 3600.0 (60 units squared)
    pub max_interaction_distance_squared: Distance,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity_y: num!(GRAVITY_Y),
            wall_bounce_damping: num!(WALL_BOUNCE_DAMPING),
            peg_bounce_damping: num!(PEG_BOUNCE_DAMPING),
            interaction_force_strength: num!(INTERACTION_FORCE_STRENGTH),
            max_interaction_distance_squared: num!(MAX_INTERACTION_DISTANCE_SQUARED),
        }
    }
}

impl PhysicsConfig {
    /// Create a new physics configuration with validation
    /// 
    /// # Panics
    /// 
    /// Panics in debug builds if parameters are outside reasonable ranges
    #[must_use]
    pub fn new(
        gravity_y: ForceStrength,
        wall_bounce_damping: DampingFactor,
        peg_bounce_damping: DampingFactor,
        interaction_force_strength: ForceStrength,
        max_interaction_distance_squared: Distance,
    ) -> Self {
        let config = Self {
            gravity_y,
            wall_bounce_damping,
            peg_bounce_damping,
            interaction_force_strength,
            max_interaction_distance_squared,
        };
        
        config.validate();
        config
    }
    
    /// Validate physics parameters are within reasonable bounds
    /// 
    /// This helps catch configuration errors that could lead to
    /// unstable or unrealistic physics behavior.
    fn validate(&self) {
        debug_assert!(
            self.gravity_y >= num!(0.0) && self.gravity_y <= num!(1000.0),
            "Gravity should be between 0 and 1000, got: {:?}", self.gravity_y
        );
        debug_assert!(
            self.wall_bounce_damping >= num!(0.0) && self.wall_bounce_damping <= num!(1.0),
            "Wall bounce damping should be between 0.0 and 1.0, got: {:?}", self.wall_bounce_damping
        );
        debug_assert!(
            self.peg_bounce_damping >= num!(0.0) && self.peg_bounce_damping <= num!(1.0),
            "Peg bounce damping should be between 0.0 and 1.0, got: {:?}", self.peg_bounce_damping
        );
        debug_assert!(
            self.interaction_force_strength >= num!(0.0),
            "Interaction force strength should be non-negative, got: {:?}", self.interaction_force_strength
        );
        debug_assert!(
            self.max_interaction_distance_squared >= num!(0.0),
            "Max interaction distance squared should be non-negative, got: {:?}", self.max_interaction_distance_squared
        );
    }
}