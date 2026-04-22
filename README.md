# FlappyBird

A Flappy Bird clone built with Rust and WebAssembly.

## Build

```bash
cargo build --target web --out-dir web/pkg
```

## Run

Serve the `web/` directory with any HTTP server:

```bash
cd web && npx serve
```

Or use Python:

```bash
cd web && python -m http.server 8080
```

Then open http://localhost:8080 in your browser.

## Controls

- Click or press Space to jump
- Click to restart after game over