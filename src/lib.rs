use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct Game {
    width: f64,
    height: f64,
    bird_y: f64,
    bird_velocity: f64,
    pipes: Vec<Pipe>,
    score: u32,
    game_over: bool,
    frame_count: u32,
}

#[wasm_bindgen]
pub struct Pipe {
    x: f64,
    gap_y: f64,
}

const GRAVITY: f64 = 0.25;
const LIFT: f64 = -4.5;
const PIPE_SPEED: f64 = 2.0;
const PIPE_WIDTH: f64 = 50.0;
const PIPE_GAP: f64 = 120.0;
const BIRD_X: f64 = 50.0;
const BIRD_RADIUS: f64 = 12.0;

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Game {
        Game {
            width,
            height,
            bird_y: height / 2.0,
            bird_velocity: 0.0,
            pipes: Vec::new(),
            score: 0,
            game_over: false,
            frame_count: 0,
        }
    }

    #[wasm_bindgen]
    pub fn jump(&mut self) {
        if !self.game_over {
            self.bird_velocity = LIFT;
        } else {
            self.reset();
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.bird_y = self.height / 2.0;
        self.bird_velocity = 0.0;
        self.pipes.clear();
        self.score = 0;
        self.game_over = false;
        self.frame_count = 0;
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        self.bird_velocity += GRAVITY;
        self.bird_y += self.bird_velocity;

        if self.bird_y < BIRD_RADIUS || self.bird_y > self.height - BIRD_RADIUS {
            self.game_over = true;
            return;
        }

        if self.frame_count % 100 == 0 {
            let gap_y = 150.0 + (self.height - 300.0).random() as f64;
            self.pipes.push(Pipe {
                x: self.width,
                gap_y,
            });
        }

        for pipe in self.pipes.iter_mut() {
            pipe.x -= PIPE_SPEED;

            let in_pipe_x = pipe.x < BIRD_X + BIRD_RADIUS && pipe.x + PIPE_WIDTH > BIRD_X - BIRD_RADIUS;
            let in_gap = self.bird_y > pipe.gap_y - BIRD_RADIUS && self.bird_y < pipe.gap_y + PIPE_GAP + BIRD_RADIUS;

            if in_pipe_x && !in_gap {
                self.game_over = true;
                return;
            }
        }

        self.pipes.retain(|p| p.x + PIPE_WIDTH > 0.0);

        for pipe in self.pipes.iter() {
            if pipe.x + PIPE_WIDTH < BIRD_X - BIRD_RADIUS {
                self.score += 1;
            }
        }

        self.frame_count += 1;
    }

    #[wasm_bindgen]
    pub fn render(&self, context: &CanvasRenderingContext2d) {
        context.set_fill_style(&JsValue::from_str("#70c5ce"));
        context.fill_rect(0.0, 0.0, self.width, self.height);

        context.set_fill_style(&JsValue::from_str("#ded895"));
        context.fill_rect(0.0, self.height - 20.0, self.width, 20.0);

        context.set_fill_style(&JsValue::from_str("#e0b028"));
        context.begin_path();
        context.arc(BIRD_X, self.bird_y, BIRD_RADIUS, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.fill();
        context.set_fill_style(&JsValue::from_str("#fff"));
        context.begin_path();
        context.arc(BIRD_X + 4.0, self.bird_y - 4.0, 4.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.fill();

        context.set_fill_style(&JsValue::from_str("#73bf2e"));
        for pipe in &self.pipes {
            context.fill_rect(pipe.x, 0.0, PIPE_WIDTH, pipe.gap_y);
            context.fill_rect(pipe.x, pipe.gap_y + PIPE_GAP, PIPE_WIDTH, self.height - pipe.gap_y - PIPE_GAP);
        }

        context.set_font("24px sans-serif");
        context.set_fill_style(&JsValue::from_str("#fff"));
        context.fill_text(&format!("{}", self.score), 20.0, 35.0).unwrap();

        if self.game_over {
            context.set_font("48px sans-serif");
            context.fill_text("Game Over!", self.width / 2.0 - 120.0, self.height / 2.0).unwrap();
            context.set_font("20px sans-serif");
            context.fill_text("Click to restart", self.width / 2.0 - 70.0, self.height / 2.0 + 40.0).unwrap();
        }
    }

    #[wasm_bindgen]
    pub fn get_score(&self) -> u32 {
        self.score
    }

    #[wasm_bindgen]
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
}

trait Random {
    fn random(&self) -> f64;
}

impl Random for f64 {
    fn random(&self) -> f64 {
        js_sys::Math::random() * self
    }
}

#[wasm_bindgen]
pub fn create_game(width: f64, height: f64) -> Game {
    Game::new(width, height)
}