# GBA Peggle/Roguelike Deck-builder

This is a game for the Game Boy Advance written in Rust using the `agb` crate. It's a work-in-progress that combines gameplay elements from Peggle/Pachinko with a roguelike deck-builder twist.

## Current State

The game is in its early stages of development. Here's what's implemented so far:

*   **Core Gameplay Loop:** The basic gameplay loop is in place. You can aim and shoot a ball, which then falls and bounces off pegs.
*   **Physics:** A simple physics engine handles gravity and collisions between the ball and the pegs, as well as wall collisions.
*   **State Management:** The game uses a state machine to manage the different phases of gameplay (aiming, falling, counting).
*   **Graphics:** The game uses sprites for the ball and pegs, loaded from `.aseprite` files.

## Building and Running

To build the game, you'll need to have the Rust toolchain installed. You can then build the game using the following command:

```bash
cargo build
```

To run the tests, use the following command:

```bash
cargo test
```

## Contributing

This project is open to contributions. If you'd like to contribute, please feel free to fork the repository and submit a pull request.
