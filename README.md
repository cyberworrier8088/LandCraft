# LandCraft

# LandCraft

A Minecraft-inspired voxel sandbox game written in **Rust** using the **Bevy Engine**.

> **Current Version:** `v0.1 Beta`

LandCraft is an open-source voxel game focused on learning game engine development, procedural world generation, and Minecraft-style mechanics using pure Rust.

---

## Screenshot :)

> Coming Soon (because this was devlopment)

---

## Features

### Procedural World
- Infinite-style chunk loading
- Procedural terrain generation
- Perlin Noise terrain
- Grass and Cobblestone blocks
- Dynamic chunk generation around the player

### Blocks
- Break blocks
- Place blocks
- Block selection highlight
- Texture atlas rendering
- Face culling optimized chunk meshes

### Player
- First Person Camera
- Third Person Camera (F2)
- Smooth movement
- Jumping
- Gravity
- Collision detection
- Mouse look
- Cursor locking

### Inventory
- 9-slot hotbar
- Block selection
- Keyboard controls (1-9)
- UI icons

### UI
- Crosshair
- Hotbar
- Selected slot highlight

### Player Model
- Animated player
- Idle animation
- Running animation
- Camera synchronized player model

---

# Built With

- Rust
- Bevy Engine
- Bevy Embedded Assets

---

# Project Structure

```
src/
│
├── main.rs
├── player.rs
├── player_model.rs
├── world.rs
├── mesh.rs
├── noise.rs
├── inventory.rs
├── ui.rs
└── exit.rs
```

---

# Getting Started

## Clone

```bash
git clone https://github.com/cyberworrier8088/LandCraft.git
```

## Enter Project

```bash
cd LandCraft
```

## Run

```bash
cargo run
```

---

# 📦 Requirements

- Rust (Latest Stable)
- Cargo
- GPU with Vulkan/OpenGL support

---

## Future

- Multiplayer
- Weather
- Caves
- Structures
- Water
- Lava
- Villages
- Survival Mode

---

#  Contributing

Contributions are welcome!

Feel free to open issues or submit pull requests.

---

#  License

This project is licensed under the MIT License.

---

# Why LandCraft?

LandCraft is a personal project built to explore how voxel games like Minecraft work internally while learning advanced Rust programming and game engine development.

The goal is to create a lightweight, open-source voxel engine completely in Rust.

---

# Version

```
LandCraft v0.1 Beta
```

### Current Status

Playable

Implemented:

- Procedural terrain
- Chunk generation
- Player movement
- Collision system
- Inventory
- Block placing
- Block breaking
- Animated player
- First & Third person camera
- Crosshair
- Hotbar
- Procedural terrain generation

Development is active, and many more features are planned.

---

Made with in Rust.


This is underdevlopment