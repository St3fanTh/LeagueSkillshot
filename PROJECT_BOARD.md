# League Skillshot — Project Board

## Status Overview

| # | Identifier | Title | Status | Priority |
|---|-----------|-------|--------|----------|
| 1 | SOF-1 | CTO – Flappy Bird Clone (Rust + WASM) | BLOCKED | medium |
| 2 | SOF-2 | Create a new agent | BLOCKED | medium |
| 3 | SOF-3 | Create a new agent | BLOCKED | critical |
| 4 | SOF-4 | Create a new agent | BLOCKED | medium |
| 5 | SOF-5 | Create a new agent | BLOCKED | medium |
| 6 | SOF-6 | Protype | DONE | medium |
| 7 | SOF-7 | Push Git | DONE | medium |
| 8 | SOF-8 | Local | DONE | medium |
| 9 | SOF-9 | Fix the bug | BLOCKED | critical |
| 10 | SOF-10 | Physics feel clunky | BLOCKED | medium |
| 11 | SOF-11 | Git Push | DONE | low |
| 12 | SOF-12 | Prototype | BLOCKED | medium |
| 13 | SOF-13 | Prototype | IN_PROGRESS | medium |
| 14 | SOF-14 | Skillshot | BLOCKED | medium |
| 15 | SOF-15 | Enemies | BLOCKED | medium |
| 16 | SOF-16 | Collision | TODO | medium |
| 17 | SOF-17 | Collision | IN_PROGRESS | medium |

## Active Work: League Skillshot Steps

This is the active iteration — building a League of Legends-inspired skillshot trainer in Rust + WASM.

### ✅ SOF-6 — Protype (DONE)
Basic League-style game prototype with Ezreal and enemies. WASM canvas rendering.

### ✅ SOF-13 — Prototype (DONE)
**Step 1: Foundation**
- Refactored `Player` struct with `x, y, target_x, target_y`
- `update(speed)` moves toward right-click target at constant speed
- `set_target(x, y)` for right-click destination
- Canvas context menu disabled
- Fixed deprecated `set_fill_style` → `set_*_style_str`
- Clean build, zero warnings

### 🔜 SOF-14 — Skillshot (BLOCKED)
**Step 2: The Skillshot (Ezreal's Q)**
> Requirements:
> 1. Create `Projectile` struct with pos_x, pos_y, velocity_x, velocity_y, distance_traveled
> 2. 'Q' key → normalized direction vector to mouse position → spawn projectile
> 3. Projectile disappears after MAX_RANGE
> 4. 0.25s cast time before firing

### 🔜 SOF-15 — Enemies (BLOCKED)
**Step 3: Enemy AI**
> Requirements:
> 1. `Enemy` struct spawns at random positions outside canvas every 2s
> 2. Enemies move slowly toward player's x,y
> 3. Store in `Vec<Enemy>` within `GameState`
> 4. Draw as red circles

### 🔜 SOF-16 / SOF-17 — Collision (TODO / IN_PROGRESS)
**Step 4: Collision & Cleanup**
> Requirements:
> 1. Circle collision (distance formula) for projectile vs enemy → remove both
> 2. Enemy vs player → game over state
> 3. Score counter on monster hit
> 4. Efficient `Vec` removal with `retain`

### 🔚 SOF-9 — Fix the bug (BLOCKED)
Fix deprecated warnings in `src/lib.rs` from earlier prototype. Depends on SOF-13 completion.

## Project Structure

```
LeagueSkillshot/
├── Cargo.toml
├── src/
│   └── lib.rs          # Main game logic (Rust + wasm-bindgen)
├── web/
│   ├── index.html     # Game UI
│   └── pkg/          # Compiled WASM output (don't edit)
└── README.md
```

## Controls (Current)

- **Right-click** → Move player to target
- **Q key** → Shoot skillshot (planned)
- **WASD/Arrows** → Direct movement (original, to be removed)

## Build

```bash
~/.cargo/bin/wasm-pack build --target web --out-dir web/pkg
```

## Run

```bash
cd web
python3 -m http.server 9000
```