use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct Game {
    width: f64,
    height: f64,
    player_x: f64,
    player_y: f64,
    skill_shots: Vec<SkillShot>,
    enemies: Vec<Enemy>,
    keys_pressed: Vec<String>,
    score: u32,
    game_over: bool,
    frame_count: u32,
}

#[wasm_bindgen]
pub struct SkillShot {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    active: bool,
}

#[wasm_bindgen]
pub struct Enemy {
    x: f64,
    y: f64,
    hp: i32,
    active: bool,
}

const PLAYER_SPEED: f64 = 3.0;
const SHOT_SPEED: f64 = 8.0;
const SHOT_COOLDOWN: u32 = 30;
const ENEMY_SPAWN_RATE: u32 = 120;
const PLAYER_SIZE: f64 = 30.0;

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Game {
        Game {
            width,
            height,
            player_x: width / 2.0,
            player_y: height - 80.0,
            skill_shots: Vec::new(),
            enemies: Vec::new(),
            keys_pressed: Vec::new(),
            score: 0,
            game_over: false,
            frame_count: 0,
        }
    }

    #[wasm_bindgen]
    pub fn key_down(&mut self, key: String) {
        if !self.keys_pressed.contains(&key) {
            self.keys_pressed.push(key);
        }
    }

    #[wasm_bindgen]
    pub fn key_up(&mut self, key: String) {
        self.keys_pressed.retain(|k| k != &key);
    }

    #[wasm_bindgen]
    pub fn shoot(&mut self) {
        if self.game_over {
            self.reset();
            return;
        }

        if self.frame_count % SHOT_COOLDOWN == 0 {
            self.skill_shots.push(SkillShot {
                x: self.player_x,
                y: self.player_y - PLAYER_SIZE / 2.0,
                vx: 0.0,
                vy: -SHOT_SPEED,
                active: true,
            });
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.player_x = self.width / 2.0;
        self.player_y = self.height - 80.0;
        self.skill_shots.clear();
        self.enemies.clear();
        self.score = 0;
        self.game_over = false;
        self.frame_count = 0;
    }

    fn move_player(&mut self) {
        for key in &self.keys_pressed {
            match key.as_str() {
                "a" | "ArrowLeft" => {
                    if self.player_x > PLAYER_SIZE {
                        self.player_x -= PLAYER_SPEED;
                    }
                }
                "d" | "ArrowRight" => {
                    if self.player_x < self.width - PLAYER_SIZE {
                        self.player_x += PLAYER_SPEED;
                    }
                }
                "w" | "ArrowUp" => {
                    if self.player_y > PLAYER_SIZE {
                        self.player_y -= PLAYER_SPEED;
                    }
                }
                "s" | "ArrowDown" => {
                    if self.player_y < self.height - PLAYER_SIZE {
                        self.player_y += PLAYER_SPEED;
                    }
                }
                _ => {}
            }
        }
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        self.move_player();

        for shot in self.skill_shots.iter_mut() {
            if shot.active {
                shot.x += shot.vx;
                shot.y += shot.vy;
                if shot.y < 0.0 || shot.x < 0.0 || shot.x > self.width {
                    shot.active = false;
                }
            }
        }
        self.skill_shots.retain(|s| s.active);

        if self.frame_count % ENEMY_SPAWN_RATE == 0 {
            let start_x = (self.width - 40.0) * js_sys::Math::random() + 20.0;
            self.enemies.push(Enemy {
                x: start_x,
                y: -20.0,
                hp: 1,
                active: true,
            });
        }

        for enemy in self.enemies.iter_mut() {
            if enemy.active {
                enemy.y += 1.5;
                if enemy.y > self.height + 20.0 {
                    enemy.active = false;
                }
            }
        }

        for shot in self.skill_shots.iter_mut() {
            for enemy in self.enemies.iter_mut() {
                if shot.active && enemy.active {
                    let dx = shot.x - enemy.x;
                    let dy = shot.y - enemy.y;
                    if dx * dx < 900.0 && dy * dy < 900.0 {
                        enemy.hp -= 1;
                        shot.active = false;
                        if enemy.hp <= 0 {
                            enemy.active = false;
                            self.score += 10;
                        }
                    }
                }
            }
        }

        for enemy in &self.enemies {
            if enemy.active {
                let dx = self.player_x - enemy.x;
                let dy = self.player_y - enemy.y;
                if dx * dx < 900.0 && dy * dy < 900.0 {
                    self.game_over = true;
                    return;
                }
            }
        }

        self.enemies.retain(|e| e.active);
        self.frame_count += 1;
    }

    #[wasm_bindgen]
    pub fn render(&self, context: &CanvasRenderingContext2d) {
        context.set_fill_style(&JsValue::from_str("#1a1a2e"));
        context.fill_rect(0.0, 0.0, self.width, self.height);

        context.set_fill_style(&JsValue::from_str("#16213e"));
for i in 0..5 {
                context.fill_rect(0.0, self.height / 5.0 * (i as f64) + 10.0, self.width, 2.0);
        }

        context.set_fill_style(&JsValue::from_str("#0f3460"));
        context.fill_rect(0.0, self.height - 40.0, self.width, 40.0);

        let ezreal_color = if self.game_over { "#888" } else { "#00b4d8" };
        context.set_fill_style(&JsValue::from_str(ezreal_color));
        context.begin_path();
        context.rect(
            self.player_x - PLAYER_SIZE / 2.0,
            self.player_y - PLAYER_SIZE / 2.0,
            PLAYER_SIZE,
            PLAYER_SIZE,
        );
        context.fill();

        context.set_fill_style(&JsValue::from_str("#e94560"));
        for enemy in &self.enemies {
            if enemy.active {
                context.begin_path();
                context.arc(enemy.x, enemy.y, 15.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
                context.fill();
            }
        }

        context.set_fill_style(&JsValue::from_str("#90e0ef"));
        context.set_stroke_style(&JsValue::from_str("#00b4d8"));
        context.set_line_width(2.0);
        for shot in &self.skill_shots {
            if shot.active {
                context.begin_path();
                context.arc(shot.x, shot.y, 5.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
                context.stroke();
            }
        }

        context.set_font("16px sans-serif");
        context.set_fill_style(&JsValue::from_str("#fff"));
        context.fill_text(&format!("Score: {}", self.score), 10.0, 25.0).unwrap();

        if self.game_over {
            context.set_font("32px sans-serif");
            context.set_fill_style(&JsValue::from_str("#e94560"));
            context.fill_text("Game Over!", self.width / 2.0 - 90.0, self.height / 2.0).unwrap();
            context.set_font("16px sans-serif");
            context.set_fill_style(&JsValue::from_str("#fff"));
            context.fill_text("Press Q or Click to restart", self.width / 2.0 - 110.0, self.height / 2.0 + 35.0).unwrap();
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

#[wasm_bindgen]
pub fn create_game(width: f64, height: f64) -> Game {
    Game::new(width, height)
}