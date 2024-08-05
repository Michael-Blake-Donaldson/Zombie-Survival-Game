# Zombie Survival Game

A simple zombie survival game created using Rust and the `pixels` crate. The game features a player character that must avoid zombies for as long as possible. As the game progresses, more zombies spawn, and the spawn rate increases, making it progressively more challenging.

## Gameplay

- **Player:** Use `W`, `A`, `S`, `D` keys to move the player around the screen.
- **Zombies:** Red zombies move at a standard speed, while blue zombies move faster.
- **Objective:** Avoid colliding with zombies to survive as long as possible. The player's health decreases upon collision, and the game ends when the health reaches zero.
- **Score:** The score increases over time based on how long the player survives.

## Features

- **Dynamic Zombie Spawning:** Zombies spawn at the edges of the screen and their spawn rate increases over time.
- **Different Zombie Types:** The game includes both regular (red) zombies and faster (blue) zombies.
- **Collision Detection:** The player takes damage upon colliding with any zombie.
- **Game Over Screen:** When the player's health reaches zero, a game over screen is displayed.

## Technologies Used

- **Rust:** The primary programming language used for game logic and implementation.
- **Pixels:** A crate for rendering 2D graphics.
- **Winit:** A crate for window creation and handling input events.
- **Rusttype:** A crate for rendering text using TrueType fonts.
- **Rand:** A crate for random number generation.
