# dice_roller_rs

A Rust library for simulating dice rolls in RPG-style games.

## Features

- Roll multiple dice with modifiers (e.g., 2d6+3)
- Simple single die rolls
- Roll with advantage (D&D 5e style - roll twice, take higher)
- Roll with disadvantage (roll twice, take lower)
- Detailed roll results with individual die values

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dice_roller_rs = "0.1.0"
```

## Usage

```rust
use dice_roller_rs::{roll, roll_simple, roll_advantage, roll_disadvantage};

fn main() {
    // Roll 2d6+3 (two six-sided dice plus 3)
    let result = roll(2, 6, 3);
    println!("{}", result); // e.g., "2d6+3: [4, 5] = 12"
    
    // Access individual components
    println!("Rolls: {:?}", result.rolls);
    println!("Total: {}", result.total);
    
    // Roll a simple d20
    let d20 = roll_simple(20);
    println!("d20: {}", d20);
    
    // Roll with advantage
    let advantage = roll_advantage(20);
    println!("d20 with advantage: {}", advantage);
    
    // Roll with disadvantage
    let disadvantage = roll_disadvantage(20);
    println!("d20 with disadvantage: {}", disadvantage);
}
```

## API

### `roll(num_dice: u32, sides: u32, modifier: i32) -> DiceRoll`

Roll multiple dice with a modifier. Returns a `DiceRoll` struct containing:
- `num_dice`: Number of dice rolled
- `sides`: Number of sides on each die
- `modifier`: Modifier added to the total
- `rolls`: Vector of individual roll results
- `total`: Sum of rolls plus modifier

### `roll_simple(sides: u32) -> u32`

Roll a single die and return the result.

### `roll_advantage(sides: u32) -> u32`

Roll twice and return the higher result.

### `roll_disadvantage(sides: u32) -> u32`

Roll twice and return the lower result.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
