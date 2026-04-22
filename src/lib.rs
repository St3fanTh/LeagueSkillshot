use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

const SKILLSHOT_SPEED: f64 = 12.0;

#[wasm_bindgen]
pub struct Skillshot {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    active: bool,
}

#[wasm_bindgen]
impl Skillshot {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, target_x: f64, target_y: f64) -> Skillshot {
        let dx = target_x - x;
        let dy = target_y - y;
        let dist = (dx * dx + dy * dy).sqrt();
        let (vx, vy) = if dist > 0.0 {
            ((dx / dist) * SKILLSHOT_SPEED, (dy / dist) * SKILLSHOT_SPEED)
        } else {
            (0.0, -SKILLSHOT_SPEED)
        };
        Skillshot { x, y, vx, vy, active: true }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_vx(&self) -> f64 {
        self.vx
    }

    pub fn get_vy(&self) -> f64 {
        self.vy
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn update(&mut self) {
        if !self.active {
            return;
        }
        self.x += self.vx;
        self.y += self.vy;
        if self.x < 0.0 || self.x > 400.0 || self.y < 0.0 || self.y > 600.0 {
            self.active = false;
        }
    }

    pub fn render(&self, context: &CanvasRenderingContext2d) {
        if !self.active {
            return;
        }
        context.set_fill_style_str("#00ff88");
        context.begin_path();
        context.ellipse(self.x, self.y, 8.0, 4.0, self.vy.atan2(self.vx), 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.fill();
    }
}

#[wasm_bindgen]
pub struct Enemy {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    hp: i32,
    active: bool,
}

#[wasm_bindgen]
impl Enemy {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Enemy {
        Enemy {
            x,
            y,
            vx: 0.0,
            vy: 1.5,
            hp: 1,
            active: true,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_hp(&self) -> i32 {
        self.hp
    }

    pub fn take_damage(&mut self) -> bool {
        self.hp -= 1;
        if self.hp <= 0 {
            self.active = false;
        }
        self.active
    }

    pub fn update(&mut self) {
        if !self.active {
            return;
        }
        self.x += self.vx;
        self.y += self.vy;
        if self.y > 620.0 {
            self.active = false;
        }
    }

    pub fn render(&self, context: &CanvasRenderingContext2d) {
        if !self.active {
            return;
        }
        context.set_fill_style_str("#ff4757");
        context.begin_path();
        context.arc(self.x, self.y, 12.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.fill();
        
        context.set_stroke_style_str("#ff6b81");
        context.begin_path();
        context.arc(self.x, self.y, 12.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.stroke();
    }
}

#[wasm_bindgen]
pub fn check_collision(skillshot: &Skillshot, enemy: &Enemy) -> bool {
    if !skillshot.is_active() || !enemy.is_active() {
        return false;
    }

    let sx = skillshot.get_x();
    let sy = skillshot.get_y();
    let ex = enemy.get_x();
    let ey = enemy.get_y();

    let rx = 8.0;
    let ry = 4.0;
    let enemy_radius = 12.0;

    let angle = skillshot.get_vy().atan2(skillshot.get_vx());
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let dx = ex - sx;
    let dy = ey - sy;

    let local_x = dx * cos_a + dy * sin_a;
    let local_y = -dx * sin_a + dy * cos_a;

    let closest_x = if local_x >= 0.0 { local_x.min(rx) } else { -(-local_x).min(rx) };
    let closest_y = if local_y >= 0.0 { local_y.min(ry) } else { -(-local_y).min(ry) };

    let dist_sq = (local_x - closest_x).powi(2) + (local_y - closest_y).powi(2);
    dist_sq <= enemy_radius * enemy_radius
}

#[wasm_bindgen]
pub struct Player {
    x: f64,
    y: f64,
    target_x: f64,
    target_y: f64,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Player {
        Player {
            x,
            y,
            target_x: x,
            target_y: y,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    #[wasm_bindgen]
    pub fn set_target(&mut self, target_x: f64, target_y: f64) {
        self.target_x = target_x;
        self.target_y = target_y;
    }

    #[wasm_bindgen]
    pub fn update(&mut self, speed: f64) {
        let dx = self.target_x - self.x;
        let dy = self.target_y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > speed {
            self.x += (dx / dist) * speed;
            self.y += (dy / dist) * speed;
        } else {
            self.x = self.target_x;
            self.y = self.target_y;
        }
    }

    #[wasm_bindgen]
    pub fn render(&self, context: &CanvasRenderingContext2d) {
        context.set_fill_style_str("#16213e");
        context.fill_rect(0.0, 0.0, 400.0, 600.0);

        context.set_fill_style_str("#0f3460");
        context.fill_rect(0.0, 560.0, 400.0, 40.0);

        context.set_fill_style_str("#00b4d8");
        context.begin_path();
        context.arc(self.x, self.y, 15.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.fill();

        context.set_stroke_style_str("#90e0ef");
        context.begin_path();
        context.arc(self.x, self.y, 15.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        context.stroke();
    }
}

#[wasm_bindgen]
pub fn create_player(x: f64, y: f64) -> Player {
    Player::new(x, y)
}