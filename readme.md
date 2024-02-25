# battlecity-rs

Simple 4 player Battle City game clone written in rust

This project is a rewrite of my 10 years old project [BattleCity](https://github.com/PsychoSanchez/BattleCity) written in C++.

It uses the [piston graphics](https://github.com/PistonDevelopers/graphics) for rendering game window, graphics and handling events. The game logic is written in pure rust. Project is cross-platform and should work on Windows, Linux and MacOS.

## How to build

### Prerequisites

- Rust 1.51.0 or later
- Cargo

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```


## How to play

### Controls

- Player 1: `↑`, `←`, `↓`, `→` to move, `Spacebar` to shoot
- Player 2: `W`, `A`, `S`, `D` to move, `C` to shoot
- Player 3: `T`, `F`, `G`, `H` to move, `B` to shoot
- Player 4: `I`, `J`, `K`, `L` to move, `M` to shoot

- Press `R` to restart the game

### Power-ups

- Armor (A): Increases the player's armor by 1, up to a maximum of 3
- Health (H): Restores the player's health to full, up to a maximum of 3


## Screenshots

![screenshot](https://raw.githubusercontent.com/PsychoSanchez/battlecity-rs/main/readme/game.gif)
