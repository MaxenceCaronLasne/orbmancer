# Complete Rust `agb` Crate Manual for AI Code Generation

The Rust `agb` (A Game Boy Advance) crate is a comprehensive, high-level library for developing games on the Game Boy Advance using modern Rust practices. This manual provides AI assistants with detailed information needed to generate accurate, efficient agb-based code.

## What is agb and why it matters

**agb transforms Game Boy Advance development** by providing safe, high-level abstractions over GBA hardware without sacrificing performance. Unlike traditional GBA development requiring deep assembly knowledge and manual memory management, agb leverages Rust's type system to prevent common hardware errors while maintaining zero-cost abstractions.

The crate handles complex hardware interactions through a central `Gba` struct that safely manages display, audio, input, and system resources. **Key advantages include**: compile-time asset conversion, built-in high-performance audio mixer, comprehensive sprite and background systems, and seamless integration with modern Rust tooling. The library supports both emulator development and real hardware deployment, making it suitable for hobbyist game development, educational projects, and commercial GBA game creation.

**Hardware context**: The Game Boy Advance features a 16.78 MHz ARM7TDMI processor, 32KB internal RAM, 256KB external RAM, and a 240×160 pixel screen. These constraints require careful memory management and performance optimization, which agb handles through specialized allocators, fixed-point arithmetic, and timing-aware APIs.

## Installation and setup requirements

### Prerequisites and toolchain

**Essential components** for agb development include Rust nightly toolchain, ARM cross-compilation tools, and the thumbv4t-none-eabi target. The build system requires `build-std` capability since thumbv4t-none-eabi is a Tier 3 target without pre-built standard library support.

**Platform-specific dependencies**:

- **Windows**: GNU Arm Embedded Toolchain with PATH environment variable configuration
- **Linux (Debian/Ubuntu)**: `sudo apt install binutils-arm-none-eabi`
- **Arch Linux**: `sudo pacman -S arm-none-eabi-binutils`

**Essential tools**:

```bash
rustup toolchain install nightly
rustup default nightly
cargo install agb-gbafix
```

### Project configuration

**Cargo.toml setup**:

```toml
[package]
name = "your-game"
version = "0.1.0"
edition = "2021"

[dependencies]
agb = "0.21"
```

**Target configuration** (`.cargo/config.toml`):

```toml
[build]
target = "thumbv4t-none-eabi"

[target.thumbv4t-none-eabi]
rustflags = ["-Clink-arg=-Tlinker_scripts/normal_boot.ld"]

[unstable]
build-std = ["core"]
```

**Recommended project structure**:

```
your-project/
├── Cargo.toml
├── .cargo/config.toml
├── src/main.rs
├── linker_scripts/normal_boot.ld
├── assets/
└── target/
```

## Core API reference and architecture

### Entry point and main structure

**Every agb program** begins with the `#[agb::entry]` attribute macro that handles hardware initialization and provides the central `Gba` struct:

```rust
#![no_std]
#![no_main]

use agb::Gba;

#[agb::entry]
fn main(mut gba: Gba) -> ! {
    // Game initialization
    loop {
        // Main game loop
        agb::halt(); // Wait for next frame
    }
}
```

**The `Gba` struct** provides safe access to all hardware subsystems:

```rust
pub struct Gba {
    pub graphics: GraphicsDist,     // Display hardware
    pub mixer: MixerController,     // Audio system
    pub save: SaveManager,          // Save data
    pub timers: TimerController,    // Hardware timers
}
```

### Display module (`agb::display`)

**Graphics architecture** centers around the `GraphicsFrame` pattern for safe rendering:

```rust
let mut gfx = gba.graphics.get();
loop {
    game.update();
    let mut frame = gfx.frame();
    game.render(&mut frame);  // All rendering operations
    frame.commit();          // Apply changes and wait for VBlank
}
```

**Sprite system** (`agb::display::object`):

- Supports up to 128 sprites with sizes from 8×8 to 64×64 pixels
- **Z-ordering and transformations**: Sprites support depth layering and affine transformations (rotation/scaling)
- **Memory management**: Automatic VRAM allocation and deallocation

```rust
use agb::display::object::ObjectController;

let mut obj = ObjectController::new();
let sprite_id = obj.sprite(&SPRITE_DATA);
obj.set_position(sprite_id, (100, 50));
obj.show(sprite_id);
```

**Background system**:

- **Four background layers** in tiled mode (mode 0)
- **Tile-based rendering** using 8×8 pixel tiles
- **Scrolling and transformation** support

```rust
use agb::display::tiled::{TileFormat, TiledMap};

let mut bg = gfx.background(Priority::P0, TileFormat::FourBpp);
bg.set_tile_data(&TILE_DATA);
bg.set_map(&MAP_DATA);
bg.set_scroll_pos((scroll_x, scroll_y));
bg.show();
```

**Color system**:

- **`Rgb15` type**: Native GBA 15-bit color format
- **Palette management**: Support for 16-color and 256-color palettes
- **Color conversion**: `Rgb` type for standard 24-bit RGB values

### Audio module (`agb::sound`)

**Dual audio architecture**:

1. **Direct Sound Mixer**: High-quality WAV file playback and mixing
2. **DMG Sound**: Classic Game Boy sound channels for retro effects

**Audio mixer setup**:

```rust
use agb::sound::mixer::{Mixer, Frequency};

let mut mixer = gba.mixer.mixer(Frequency::Hz32768);
mixer.enable();

// Sound effect playback
let channel = mixer.play_sound(&SOUND_DATA);
channel.set_volume(0.5);
```

**Tracker music support**:

```rust
use agb_tracker::{include_xm, Track, Tracker};

const MUSIC: Track = include_xm!("music.xm");
let mut tracker = Tracker::new(&MUSIC);

// In main loop
tracker.step(&mut mixer);
```

### Input module (`agb::input`)

**Button handling** supports all 10 GBA buttons with state tracking:

```rust
use agb::input::{ButtonController, Button};

let input = ButtonController::new();

if input.is_pressed(Button::A) {
    // Handle A button press (single frame)
}
if input.is_held(Button::RIGHT) {
    // Handle continuous right press
}
if input.is_released(Button::START) {
    // Handle button release
}
```

**Available buttons**: A, B, L, R, START, SELECT, UP, DOWN, LEFT, RIGHT

### Fixed-point mathematics (`agb::fixnum`)

**Essential for GBA performance** since no floating-point unit exists:

```rust
use agb::fixnum::Num;

// 24.8 fixed-point number (24 integer bits, 8 fractional bits)
type FixedPoint = Num<i32, 8>;

let position = FixedPoint::new(10.5);
let velocity = FixedPoint::new(2.25);
let new_position = position + velocity;

// Integration with standard operations
let result = position * velocity / FixedPoint::new(2);
```

**Common fixed-point types**:

- `Num<i32, 8>`: General purpose decimal calculations
- `Num<i16, 8>`: Memory-efficient for simple values
- `Num<i32, 16>`: Higher precision for complex calculations

### Memory management modules

**Allocator selection**:

```rust
use agb::{ExternalAllocator, InternalAllocator};

// Default heap allocator (EWRAM - 256KB, slower)
#[global_allocator]
static HEAP: ExternalAllocator = ExternalAllocator;

// Alternative: Internal allocator (IWRAM - 32KB, faster)
#[global_allocator]
static HEAP: InternalAllocator = InternalAllocator;
```

**Memory regions and performance characteristics**:

- **IWRAM (32KB)**: 1-cycle access, use for critical code and data
- **EWRAM (256KB)**: 3-cycle access, use for general heap allocation
- **ROM (variable)**: 5-cycle access, read-only game code and assets

## Asset import and conversion system

### Compile-time asset loading

**agb's asset macros** convert graphics and audio at compile time, ensuring optimal format and performance:

**Sprite import**:

```rust
use agb::include_aseprite;

// Import Aseprite file with animation support
include_aseprite!("sprites.aseprite");
let sprite_data = sprites::player_idle;

// Multiple animations from single file
let walking_animation = sprites::player_walk;
let jumping_animation = sprites::player_jump;
```

**Background import**:

```rust
use agb::include_background_gfx;

include_background_gfx!(backgrounds,
    tiles => "tiles.png",
    map => "level1.tmx"
);

let tile_data = backgrounds::tiles;
let map_data = backgrounds::level1;
```

**Audio import**:

```rust
use agb::include_wav;

include_wav!("jump_sound.wav");
let jump_sound = jump_sound_wav;

// Tracker music
use agb_tracker::include_xm;
include_xm!("background_music.xm");
```

**Font import**:

```rust
use agb::include_font;

include_font!("font.ttf", 12); // 12-point font size
let font_data = font;
```

### Asset optimization guidelines

**Image format recommendations**:

- **4-bit sprites**: Use for most sprites to save VRAM (16 colors per palette)
- **8-bit sprites**: Only when 256 colors needed (uses double VRAM)
- **PNG format**: Preferred input format for automatic conversion
- **Aseprite integration**: Direct support for .ase/.aseprite files with animation data

**Audio format guidelines**:

- **WAV format**: Use 44.1kHz or 22kHz sample rate for sound effects
- **XM tracker files**: Preferred for background music (compact, high quality)
- **Sample size**: Keep sound effects under 1-2 seconds to conserve ROM space

## Game development patterns and examples

### Standard game loop structure

**Core game loop** follows update-render-wait pattern with VBlank synchronization:

```rust
#[agb::entry]
fn main(mut gba: Gba) -> ! {
    let mut game_state = GameState::new();
    let mut gfx = gba.graphics.get();
    let input = ButtonController::new();
    let vblank = VBlank::get();

    loop {
        // Update game logic (CPU time)
        input.update();
        game_state.update(&input);

        // Render frame (VBlank-synchronized)
        vblank.wait_for_vblank();
        let mut frame = gfx.frame();
        game_state.render(&mut frame);
        frame.commit();
    }
}
```

### Sprite animation patterns

**Frame-based animation** using Aseprite data:

```rust
struct AnimatedSprite {
    current_frame: usize,
    frame_timer: u32,
    animation_speed: u32,
}

impl AnimatedSprite {
    fn update(&mut self) {
        self.frame_timer += 1;
        if self.frame_timer >= self.animation_speed {
            self.frame_timer = 0;
            self.current_frame = (self.current_frame + 1) % SPRITE_FRAMES.len();
        }
    }

    fn render(&self, objects: &mut ObjectController) {
        let sprite_data = &SPRITE_FRAMES[self.current_frame];
        objects.set_sprite_data(self.sprite_id, sprite_data);
    }
}
```

### Entity management patterns

**Object pooling** for memory efficiency:

```rust
use heapless::Vec;

struct EntityPool<T, const N: usize> {
    entities: [Option<T>; N],
    active_count: usize,
}

impl<T: Default, const N: usize> EntityPool<T, N> {
    fn spawn(&mut self) -> Option<&mut T> {
        for entity in &mut self.entities {
            if entity.is_none() {
                *entity = Some(T::default());
                self.active_count += 1;
                return entity.as_mut();
            }
        }
        None // Pool exhausted
    }

    fn despawn(&mut self, index: usize) {
        if self.entities[index].is_some() {
            self.entities[index] = None;
            self.active_count -= 1;
        }
    }
}
```

### State management patterns

**Game state machine** for menu/gameplay transitions:

```rust
enum GameState {
    MainMenu(MenuState),
    Playing(PlayState),
    GameOver(GameOverState),
}

impl GameState {
    fn update(&mut self, input: &ButtonController) -> Option<GameState> {
        match self {
            GameState::MainMenu(menu) => {
                if input.is_pressed(Button::START) {
                    Some(GameState::Playing(PlayState::new()))
                } else {
                    menu.update(input);
                    None
                }
            }
            GameState::Playing(play) => {
                if play.player_dead() {
                    Some(GameState::GameOver(GameOverState::new()))
                } else {
                    play.update(input);
                    None
                }
            }
            GameState::GameOver(game_over) => {
                if input.is_pressed(Button::A) {
                    Some(GameState::MainMenu(MenuState::new()))
                } else {
                    game_over.update(input);
                    None
                }
            }
        }
    }
}
```

## Memory management and performance optimization

### GBA hardware constraints

**Critical memory limitations** that affect all design decisions:

- **Total RAM**: 32KB internal (fast) + 256KB external (slower)
- **CPU speed**: 16.78 MHz with no cache or floating-point unit
- **Memory access costs**: IWRAM (1 cycle), EWRAM (3 cycles), ROM (5 cycles)
- **Graphics memory**: 96KB VRAM with restricted access timing

### Memory allocation strategies

**Static allocation preferred** for predictable performance:

```rust
// Preferred: Static allocation with known size
static mut GAME_ENTITIES: [Entity; 256] = [Entity::default(); 256];
static mut SPRITE_BUFFER: [SpriteData; 128] = [SpriteData::default(); 128];

// Use heapless collections for fixed-capacity containers
use heapless::{Vec, FnvIndexMap};
let mut particles: Vec<Particle, 64> = Vec::new();
let mut lookup_table: FnvIndexMap<EntityId, usize, 32> = FnvIndexMap::new();

// Dynamic allocation as last resort
use alloc::vec::Vec;
let dynamic_data: Vec<u8> = Vec::with_capacity(estimated_size);
```

**Memory budgeting guidelines**:

- **IWRAM (32KB)**: Reserve 8KB for stack, 24KB for critical game data
- **EWRAM (256KB)**: Main heap allocation, ~200KB usable after overhead
- **VRAM (96KB)**: Tile and sprite data, carefully managed

### Performance optimization techniques

**Fixed-point arithmetic** essential for mathematical operations:

```rust
use agb::fixnum::Num;

// Pre-calculate expensive operations using lookup tables
static SINE_TABLE: [Num<i32, 8>; 256] = [...]; // Pre-computed
static COSINE_TABLE: [Num<i32, 8>; 256] = [...];

fn fast_sine(angle: u8) -> Num<i32, 8> {
    SINE_TABLE[angle as usize]
}

// Avoid division, use bit shifts when possible
let result = value >> 3; // Equivalent to value / 8
let scaled = value << 2; // Equivalent to value * 4
```

**VBlank synchronization** prevents screen tearing and glitches:

```rust
use agb::interrupt::VBlank;

fn optimized_render_loop(gfx: &mut Graphics) {
    // Game logic runs during screen draw (most of frame)
    update_game_state();
    prepare_render_data();

    // Wait for VBlank before graphics operations
    VBlank::wait_for_vblank();

    // Graphics updates during VBlank period (~12% of frame)
    update_sprite_positions();
    update_background_scroll();

    // Hardware automatically starts next frame
}
```

**DMA for bulk operations** provides 10x speed improvement:

```rust
use agb::dma::dma3;

// Use DMA for large memory transfers
dma3.copy16(&source_data, &mut dest_buffer);

// Schedule DMA during safe periods
dma3.hblank_transfer(&background_line_data);
```

### Common performance pitfalls

**Avoid these expensive operations**:

```rust
// DON'T: Floating-point operations (extremely slow)
let result = 3.14159 * radius * radius;

// DO: Fixed-point arithmetic
let pi = Num::<i32, 8>::new(3.14159);
let result = pi * radius * radius;

// DON'T: Access VRAM during active display
*(VRAM_BASE as *mut u16) = color; // Can cause display corruption

// DO: Synchronize with VBlank
VBlank::wait_for_vblank();
// Now safe to update graphics

// DON'T: Frequent heap allocation in game loop
for _ in 0..100 {
    let temp_vec = Vec::new(); // Expensive!
}

// DO: Pre-allocate or use object pools
static mut TEMP_BUFFER: [u8; 1024] = [0; 1024];
```

## Build configuration and deployment

### Development build process

**Standard development workflow**:

```bash
# Development build (slow, debugging enabled)
cargo build

# Release build (essential for performance testing)
cargo build --release

# Run in emulator (mgba preferred)
cargo run --release

# Build with standard library from source (required for Tier 3 target)
cargo build -Z build-std=core --release
```

### Hardware deployment process

**Converting to GBA ROM format**:

```bash
# Step 1: Build release binary
cargo build --release

# Step 2: Convert to .gba format using agb-gbafix
agb-gbafix target/thumbv4t-none-eabi/release/your-game -o your-game.gba

# Alternative using ARM toolchain
arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/your-game your-game.gba
```

**Hardware compatibility**:

- **Flash carts**: EverDrive GBA X5, EZ-Flash series
- **Original hardware**: Real GBA, GBA SP, Game Boy Player
- **Modern systems**: 3DS via open_agb_firm (some limitations)

### Emulator configuration

**Recommended development setup**:

- **Primary**: mGBA with `agb::println!` debugging support
- **Testing**: Multiple emulators for compatibility verification
- **Performance**: Real hardware testing for final validation

```rust
// Debug output (only works in mGBA)
agb::println!("Player position: ({}, {})", player.x, player.y);
agb::println!("Frame time: {} cycles", frame_time);
```

## Integration with Rust ecosystem

### Core agb ecosystem crates

**Essential companion crates**:

- **`agb-fixnum`**: Fixed-point arithmetic library
- **`agb-hashmap`**: no_std HashMap optimized for GBA constraints
- **`agb-image-converter`**: PNG/Aseprite to GBA format conversion
- **`agb-sound-converter`**: WAV to GBA audio format conversion
- **`agb-tracker`**: XM/MOD tracker music support
- **`agb-gbafix`**: ROM header fixing utility

### External crate integration

**Compatible no_std crates**:

```toml
[dependencies]
agb = "0.21"
heapless = "0.8"           # Collections without allocation
nb = "1.0"                 # Non-blocking I/O traits
embedded-hal = "0.2"       # Hardware abstraction layer
rand = { version = "0.8", default-features = false }
serde = { version = "1.0", default-features = false }
```

**Integration patterns**:

```rust
// Using heapless for fixed-capacity collections
use heapless::{Vec, FnvIndexMap};

struct GameWorld {
    entities: Vec<Entity, 256>,
    entity_map: FnvIndexMap<EntityId, usize, 256>,
}

// Using embedded-hal traits for hardware abstraction
use embedded_hal::digital::v2::OutputPin;

trait LedControl {
    fn set_led(&mut self, enabled: bool);
}
```

## Testing and debugging approaches

### Unit testing framework

**agb supports standard Rust testing** with emulator integration:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_movement() {
        let mut player = Player::new();
        player.update_position(Vector2::new(5, 0));
        assert_eq!(player.position.x, 5);
    }

    #[test_case]
    fn test_collision_detection(gba: &mut Gba) {
        // Test requiring GBA hardware access
        let mut game = GameState::new();
        game.setup(gba);
        // Test game logic...
    }
}
```

**Running tests**:

```bash
# Standard unit tests
cargo test

# Hardware-dependent tests via emulator
cargo test --features="testing"
```

### Debugging tools and techniques

**Debug output and logging**:

```rust
// Development logging (mGBA only)
agb::println!("Debug: Player health = {}", player.health);

// Performance monitoring
let start_time = agb::timer::timer0().value();
expensive_operation();
let elapsed = agb::timer::timer0().value() - start_time;
agb::println!("Operation took {} cycles", elapsed);
```

**Error handling patterns**:

```rust
// Result-based error handling for fallible operations
enum GameError {
    OutOfMemory,
    InvalidAsset,
    SaveDataCorrupted,
}

fn load_level(level_id: u8) -> Result<Level, GameError> {
    let level_data = LEVEL_DATA.get(level_id as usize)
        .ok_or(GameError::InvalidAsset)?;

    Level::parse(level_data)
        .ok_or(GameError::OutOfMemory)
}

// Panic handling for unrecoverable errors
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    agb::println!("Panic: {}", info);
    loop {}
}
```

### Performance profiling

**Frame timing analysis**:

```rust
struct PerformanceMonitor {
    frame_times: [u32; 60], // Track last 60 frame times
    current_frame: usize,
}

impl PerformanceMonitor {
    fn start_frame(&mut self) -> u32 {
        agb::timer::timer0().value()
    }

    fn end_frame(&mut self, start_time: u32) {
        let frame_time = agb::timer::timer0().value() - start_time;
        self.frame_times[self.current_frame] = frame_time;
        self.current_frame = (self.current_frame + 1) % 60;

        // Warn if frame takes too long (>80% of 16.67ms budget)
        if frame_time > 280000 { // ~13.3ms at 16.78MHz
            agb::println!("Slow frame: {} cycles", frame_time);
        }
    }

    fn average_frame_time(&self) -> u32 {
        self.frame_times.iter().sum::<u32>() / 60
    }
}
```

## Real-world project structure and conventions

### Production-ready project organization

**Recommended directory structure** for larger projects:

```
game-project/
├── Cargo.toml
├── .cargo/config.toml
├── build.rs                    # Build script for asset processing
├── src/
│   ├── main.rs                # Entry point
│   ├── lib.rs                 # Core game library
│   ├── game/                  # Game logic modules
│   │   ├── mod.rs
│   │   ├── entities/          # Game entities
│   │   ├── systems/           # Game systems
│   │   └── levels/            # Level management
│   ├── graphics/              # Rendering code
│   ├── audio/                 # Sound management
│   └── input/                 # Input handling
├── assets/                    # Source assets
│   ├── sprites/
│   ├── backgrounds/
│   ├── audio/
│   └── fonts/
├── data/                      # Game data files
├── tools/                     # Custom build tools
└── docs/                      # Project documentation
```

### Code organization patterns

**Entity-Component-System architecture** adapted for GBA constraints:

```rust
// Component trait for data-only structs
trait Component: Default + Copy + Clone {}

#[derive(Default, Copy, Clone)]
struct Position {
    x: Num<i32, 8>,
    y: Num<i32, 8>,
}
impl Component for Position {}

#[derive(Default, Copy, Clone)]
struct Velocity {
    dx: Num<i32, 8>,
    dy: Num<i32, 8>,
}
impl Component for Velocity {}

// System functions operate on component arrays
fn movement_system(
    positions: &mut [Position],
    velocities: &[Velocity],
    active_mask: u128, // Bitset for active entities
) {
    for i in 0..128 {
        if active_mask & (1 << i) != 0 {
            positions[i].x += velocities[i].dx;
            positions[i].y += velocities[i].dy;
        }
    }
}
```

**Module organization** for clean separation of concerns:

```rust
// src/lib.rs - Public API
pub mod graphics;
pub mod audio;
pub mod input;
pub mod game;

pub use game::Game;

// src/game/mod.rs - Game module structure
pub mod entities;
pub mod systems;
pub mod levels;
pub mod state;

pub use state::GameState;
use systems::*;

// src/main.rs - Minimal entry point
#![no_std]
#![no_main]

use agb::Gba;
use game_crate::Game;

#[agb::entry]
fn main(mut gba: Gba) -> ! {
    let mut game = Game::new();
    game.run(gba)
}
```

### Asset management conventions

**Organized asset pipeline** with build-time validation:

```rust
// src/assets.rs - Centralized asset definitions
use agb::{include_aseprite, include_background_gfx, include_wav};
use agb_tracker::include_xm;

// Sprites organized by category
include_aseprite!("sprites/player.aseprite");
include_aseprite!("sprites/enemies.aseprite");
include_aseprite!("sprites/items.aseprite");

// Backgrounds by level
include_background_gfx!(level1_bg, tiles => "backgrounds/level1_tiles.png");
include_background_gfx!(level2_bg, tiles => "backgrounds/level2_tiles.png");

// Audio assets
include_wav!("audio/sfx/jump.wav");
include_wav!("audio/sfx/collect.wav");
include_xm!("audio/music/level1.xm");

// Asset registry for runtime access
pub struct Assets {
    pub player_sprites: &'static [SpriteData],
    pub enemy_sprites: &'static [SpriteData],
    pub level1_tiles: &'static TileData,
    pub jump_sound: &'static [u8],
    pub level_music: &'static Track,
}

impl Assets {
    pub const fn new() -> Self {
        Self {
            player_sprites: player::SPRITES,
            enemy_sprites: enemies::SPRITES,
            level1_tiles: level1_bg::TILES,
            jump_sound: jump_wav::DATA,
            level_music: &level1_xm::TRACK,
        }
    }
}

pub static ASSETS: Assets = Assets::new();
```

## Best practices for AI code generation

### Code generation guidelines

**When generating agb code**, AI assistants should follow these critical patterns:

1. **Always use fixed-point arithmetic** instead of floating-point for mathematical operations
2. **Include proper VBlank synchronization** for all graphics operations
3. **Prefer static allocation** over dynamic allocation for predictable performance
4. **Use appropriate error handling** with `Result` types for fallible operations
5. **Include memory-efficient data structures** like heapless collections
6. **Generate proper asset import statements** using agb's macro system
7. **Structure code for the main game loop** with update-render-wait pattern
8. **Include appropriate documentation** explaining GBA-specific constraints

**Example of well-structured generated code**:

```rust
#![no_std]
#![no_main]

use agb::{
    Gba,
    display::object::ObjectController,
    input::{ButtonController, Button},
    interrupt::VBlank,
    fixnum::Num,
    include_aseprite,
};

// Asset imports
include_aseprite!("player.aseprite");

// Fixed-point type for calculations
type FixedPoint = Num<i32, 8>;

struct Player {
    position: (FixedPoint, FixedPoint),
    velocity: (FixedPoint, FixedPoint),
    sprite_id: u16,
}

impl Player {
    fn new(objects: &mut ObjectController) -> Self {
        let sprite_id = objects.sprite(&player::idle_0);
        Self {
            position: (FixedPoint::new(120), FixedPoint::new(80)),
            velocity: (FixedPoint::new(0), FixedPoint::new(0)),
            sprite_id,
        }
    }

    fn update(&mut self, input: &ButtonController) {
        // Input handling with fixed-point arithmetic
        if input.is_held(Button::LEFT) {
            self.velocity.0 = FixedPoint::new(-2);
        } else if input.is_held(Button::RIGHT) {
            self.velocity.0 = FixedPoint::new(2);
        } else {
            self.velocity.0 = FixedPoint::new(0);
        }

        // Physics update
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        // Boundary checking
        if self.position.0 < FixedPoint::new(0) {
            self.position.0 = FixedPoint::new(0);
        }
    }

    fn render(&self, objects: &mut ObjectController) {
        objects.set_position(
            self.sprite_id,
            (self.position.0.to_raw() >> 8, self.position.1.to_raw() >> 8),
        );
    }
}

#[agb::entry]
fn main(mut gba: Gba) -> ! {
    let mut graphics = gba.graphics.get();
    let mut objects = graphics.object();
    let input = ButtonController::new();
    let vblank = VBlank::get();

    let mut player = Player::new(&mut objects);
    objects.show(player.sprite_id);

    loop {
        // Update phase
        input.update();
        player.update(&input);

        // Render phase (VBlank synchronized)
        vblank.wait_for_vblank();
        player.render(&mut objects);
    }
}
```

### Error handling and safety patterns

**Proper error handling** for common failure modes:

```rust
// Asset loading with error handling
fn load_assets() -> Result<GameAssets, AssetError> {
    let sprites = SPRITE_DATA.get().ok_or(AssetError::SpriteDataMissing)?;
    let music = MUSIC_DATA.get().ok_or(AssetError::MusicDataMissing)?;

    Ok(GameAssets { sprites, music })
}

// Safe memory access patterns
fn update_sprite_safely(
    objects: &mut ObjectController,
    sprite_id: u16,
    position: (i32, i32),
) -> Result<(), RenderError> {
    // Bounds checking
    if position.0 < 0 || position.0 > 240 || position.1 < 0 || position.1 > 160 {
        return Err(RenderError::OutOfBounds);
    }

    objects.set_position(sprite_id, position);
    Ok(())
}

// Resource management with RAII
struct ManagedSprite {
    id: u16,
    objects: *mut ObjectController, // Raw pointer for ownership
}

impl Drop for ManagedSprite {
    fn drop(&mut self) {
        unsafe {
            (*self.objects).hide(self.id);
        }
    }
}
```

This comprehensive manual provides AI assistants with the detailed information needed to generate accurate, efficient, and safe agb-based Game Boy Advance code while respecting hardware constraints and following Rust best practices.
