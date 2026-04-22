# League Skillshot

League of Legends inspired skillshot game featuring Ezreal.

## Controls
- WASD / Arrow Keys - Move Ezreal
- Q or Click - Shoot skill shot
- Enemies have 1 HP each

## Build

```bash
cargo build --target web --out-dir web/pkg
```

Or with wasm-pack:
```bash
~/.cargo/bin/wasm-pack build --target web --out-dir web/pkg
```

## Run

```bash
cd web
python3 -m http.server 9000
```

Then open http://localhost:9000

**Important:** Run from inside the `web` folder, not the root directory.