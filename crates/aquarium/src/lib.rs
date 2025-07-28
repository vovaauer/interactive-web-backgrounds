// crates/aquarium/src/lib.rs
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

// --- GodRay System ---
struct GodRay {
    x: f64, top_width: f64, bottom_width: f64, length: f64,
    blur: f64, life: f64, max_life: f64,
}
impl GodRay {
    fn reset(&mut self, canvas_width: f64, canvas_height: f64) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(-canvas_width * 0.2..canvas_width * 1.2);
        self.top_width = rng.gen_range(20.0..150.0);
        self.bottom_width = rng.gen_range(0.0..self.top_width * 0.3);
        self.length = rng.gen_range(canvas_height * 0.5..canvas_height * 1.2);
        self.blur = rng.gen_range(10.0..25.0);
        self.max_life = rng.gen_range(1200.0..1800.0);
        self.life = 0.0;
    }
    fn new(canvas_width: f64, canvas_height: f64) -> Self {
        let mut ray = Self { x: 0.0, top_width: 0.0, bottom_width: 0.0, length: 0.0, blur: 0.0, life: 0.0, max_life: 0.0 };
        ray.reset(canvas_width, canvas_height);
        ray
    }
    fn update(&mut self) { self.life += 1.0; }
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let life_progress = self.life / self.max_life;
        let alpha = js_sys::Math::sin(life_progress * std::f64::consts::PI) * 0.15;
        if alpha <= 0.0 { return; }
        ctx.save();
        ctx.set_filter(&format!("blur({}px)", self.blur));
        ctx.set_fill_style(&JsValue::from_str(&format!("rgba(210, 230, 255, {})", alpha)));
        let y_offset = -50.0;
        ctx.translate(self.x, 0.0).unwrap();
        ctx.rotate(-15.0f64.to_radians()).unwrap();
        ctx.begin_path(); ctx.move_to(-self.top_width / 2.0, y_offset); ctx.line_to(self.top_width / 2.0, y_offset); ctx.line_to(self.bottom_width / 2.0, y_offset + self.length); ctx.line_to(-self.bottom_width / 2.0, y_offset + self.length); ctx.close_path(); ctx.fill(); ctx.restore();
    }
}

// --- Bubble System ---
const BUBBLE_SOURCES: &[(f64, f64)] = &[(-80.0, -30.0), (35.0, -135.0)];
struct Bubble {
    x: f64, y: f64, original_x: f64, size: f64, speed_y: f64, wobble_angle: f64,
}
impl Bubble {
    fn new(castle_center_x: f64, castle_base_y: f64, castle_scale: f64) -> Self {
        let mut bubble = Bubble { x: 0.0, y: 0.0, original_x: 0.0, size: 0.0, speed_y: 0.0, wobble_angle: 0.0 };
        bubble.reset(castle_center_x, castle_base_y, castle_scale);
        bubble
    }
    fn reset(&mut self, castle_center_x: f64, castle_base_y: f64, castle_scale: f64) {
        let mut rng = rand::thread_rng();
        let source = BUBBLE_SOURCES.choose(&mut rng).unwrap();
        self.original_x = castle_center_x + source.0 * castle_scale + rng.gen_range(-5.0..5.0);
        self.y = castle_base_y + source.1 * castle_scale;
        self.size = rng.gen_range(1.0..5.0);
        self.speed_y = rng.gen_range(0.5..1.5);
        self.wobble_angle = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
    }
    fn update(&mut self) {
        self.y -= self.speed_y; self.wobble_angle += 0.05;
        self.x = self.original_x + self.wobble_angle.sin() * self.size * 0.5;
    }
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path(); ctx.arc(self.x, self.y, self.size, 0.0, std::f64::consts::PI * 2.0).unwrap();
        ctx.set_fill_style(&JsValue::from_str("rgba(220, 235, 255, 0.6)")); ctx.fill();
        ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.8)")); ctx.set_line_width(1.0); ctx.stroke();
    }
}

// --- Crab System ---
enum CrabState { Walking, Waiting }
struct Crab {
    x: f64, y: f64, size: f64, direction: f64, state: CrabState, state_timer: i32,
}
impl Crab {
    fn new(canvas_width: f64, canvas_height: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0.0..canvas_width), y: canvas_height * 0.9,
            size: rng.gen_range(10.0..15.0), direction: if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
            state: CrabState::Walking, state_timer: rng.gen_range(100..300),
        }
    }
    fn update(&mut self, floor_y: f64, width: f64) {
        self.state_timer -= 1;
        if self.state_timer <= 0 {
            let mut rng = rand::thread_rng();
            match self.state {
                CrabState::Walking => { self.state = CrabState::Waiting; self.state_timer = rng.gen_range(60..180); }
                CrabState::Waiting => { self.state = CrabState::Walking; self.state_timer = rng.gen_range(100..300); }
            }
        }
        if let CrabState::Walking = self.state { self.x += self.direction * 0.5; }
        self.y = floor_y - 8.0;
        if (self.x > width && self.direction > 0.0) || (self.x < 0.0 && self.direction < 0.0) {
            self.direction *= -1.0; self.state = CrabState::Walking; self.state_timer = rand::thread_rng().gen_range(100..300);
        }
    }
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save(); ctx.translate(self.x, self.y).unwrap();
        ctx.set_fill_style(&JsValue::from_str("#d14124"));
        ctx.begin_path(); ctx.arc(0.0, 0.0, self.size, std::f64::consts::PI, 0.0).unwrap(); ctx.close_path(); ctx.fill();
        ctx.set_stroke_style(&JsValue::from_str("#d14124")); ctx.set_line_width(2.0);
        for i in 0..3 {
            let angle = (i as f64 * 0.5) + 0.2;
            ctx.begin_path(); ctx.move_to(-self.size, 0.0); ctx.line_to(-self.size * 1.5, angle * 10.0); ctx.stroke();
            ctx.begin_path(); ctx.move_to(self.size, 0.0); ctx.line_to(self.size * 1.5, angle * 10.0); ctx.stroke();
        }
        ctx.restore();
    }
}

// --- Fish and Food System ---
#[derive(Clone)] struct Food { x: f64, y: f64, vy: f64, }
#[derive(Clone)]
struct Fish {
    x: f64, y: f64, vx: f64, vy: f64, ax: f64, ay: f64,
    size: f64, color: String, wander_angle: f64, max_speed: f64, max_force: f64,
}
impl Fish {
    fn new(canvas_width: f64, canvas_height: f64) -> Self {
        let mut rng = rand::thread_rng(); let size = rng.gen_range(10.0..18.0);
        Self {
            x: rng.gen_range(0.0..canvas_width), y: rng.gen_range(0.0..canvas_height * 0.8), vx: 0.0, vy: 0.0, ax: 0.0, ay: 0.0, size,
            color: format!("hsl({}, 80%, 70%)", rng.gen_range(0..360)), wander_angle: rng.gen_range(0.0..std::f64::consts::PI * 2.0),
            max_speed: rng.gen_range(0.3..0.6), max_force: rng.gen_range(0.01..0.03),
        }
    }
    fn apply_force(&mut self, fx: f64, fy: f64) { self.ax += fx; self.ay += fy; }
    fn get_seek_force(&self, closest_food: Option<(f64, f64, f64)>, width: f64, height: f64) -> (f64, f64, f64) {
        if let Some((dist_sq, target_x, target_y)) = closest_food {
            let canvas_diagonal = (width.powi(2) + height.powi(2)).sqrt(); let dist = dist_sq.sqrt();
            let urgency = (1.0 - (dist / canvas_diagonal)).powf(2.0).clamp(0.0, 1.0);
            let mut desired_x = target_x - self.x; let mut desired_y = target_y - self.y;
            let d_mag = (desired_x * desired_x + desired_y * desired_y).sqrt();
            if d_mag > 0.0 { desired_x = (desired_x / d_mag) * self.max_speed; desired_y = (desired_y / d_mag) * self.max_speed; }
            let steer_x = desired_x - self.vx; let steer_y = desired_y - self.vy;
            let steer_mag_sq = steer_x * steer_x + steer_y * steer_y;
            if steer_mag_sq > self.max_force * self.max_force { let mag = steer_mag_sq.sqrt(); return ((steer_x / mag) * self.max_force, (steer_y / mag) * self.max_force, urgency); }
            return (steer_x, steer_y, urgency);
        }
        (0.0, 0.0, 0.0)
    }
    fn get_avoid_force(&self, floor_y: f64, width: f64, _height: f64, closest_food: Option<(f64, f64, f64)>) -> (f64, f64, f64) {
        const MARGIN: f64 = 60.0;
        let mut total_steer_x = 0.0; let mut total_steer_y = 0.0; let mut should_avoid = false;
        let (fx, fy) = if let Some((_, x, y)) = closest_food { (Some(x), Some(y)) } else { (None, None) };
        if self.x < MARGIN && !(fx.is_some() && fx.unwrap() < self.x) { total_steer_x += self.max_speed - self.vx; should_avoid = true; }
        if self.x > width - MARGIN && !(fx.is_some() && fx.unwrap() > self.x) { total_steer_x += -self.max_speed - self.vx; should_avoid = true; }
        if self.y < MARGIN && !(fy.is_some() && fy.unwrap() < self.y) { total_steer_y += self.max_speed - self.vy; should_avoid = true; }
        if self.y > floor_y - MARGIN && !(fy.is_some() && fy.unwrap() > self.y) { total_steer_y += -self.max_speed - self.vy; should_avoid = true; }
        if should_avoid {
            let mag_sq = total_steer_x * total_steer_x + total_steer_y * total_steer_y;
            if mag_sq > self.max_force * self.max_force { let mag = mag_sq.sqrt(); return ((total_steer_x / mag) * self.max_force, (total_steer_y / mag) * self.max_force, 1.0); }
            return (total_steer_x, total_steer_y, 1.0);
        }
        (0.0, 0.0, 0.0)
    }
    fn wander(&mut self) -> (f64, f64) {
        let mut rng = rand::thread_rng(); self.wander_angle += rng.gen_range(-0.3..0.3);
        let (circle_dist, circle_radius) = (50.0, 25.0);
        let norm = (self.vx * self.vx + self.vy * self.vy).sqrt();
        let (circle_center_x, circle_center_y) = if norm > 0.0 { (self.vx / norm * circle_dist, self.vy / norm * circle_dist) } else { (circle_dist, 0.0) };
        let (displacement_x, displacement_y) = (self.wander_angle.cos() * circle_radius, self.wander_angle.sin() * circle_radius);
        let (wander_force_x, wander_force_y) = (circle_center_x + displacement_x, circle_center_y + displacement_y);
        let mag = (wander_force_x * wander_force_x + wander_force_y * wander_force_y).sqrt();
        if mag > 0.0 { return ((wander_force_x / mag) * self.max_force * 0.2, (wander_force_y / mag) * self.max_force * 0.2); }
        (0.0, 0.0)
    }
    fn update(&mut self, food_items: &[Food], floor_y: f64, width: f64, height: f64) {
        let closest_food = food_items.iter().map(|f| ((self.x - f.x).powi(2) + (self.y - f.y).powi(2), f.x, f.y)).min_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let (seek_x, seek_y, seek_w) = self.get_seek_force(closest_food, width, height);
        let (avoid_x, avoid_y, avoid_w) = self.get_avoid_force(floor_y, width, height, closest_food);
        let (wander_x, wander_y) = self.wander();
        let force_x = avoid_x * avoid_w + seek_x * seek_w * (1.0 - avoid_w) + wander_x * (1.0 - seek_w) * (1.0 - avoid_w);
        let force_y = avoid_y * avoid_w + seek_y * seek_w * (1.0 - avoid_w) + wander_y * (1.0 - seek_w) * (1.0 - avoid_w);
        self.apply_force(force_x, force_y);
        self.vx += self.ax; self.vy += self.ay;
        let current_max_speed = self.max_speed + (seek_w * 0.5);
        let speed_sq = self.vx.powi(2) + self.vy.powi(2);
        if speed_sq > current_max_speed.powi(2) { let speed = speed_sq.sqrt(); self.vx = (self.vx / speed) * current_max_speed; self.vy = (self.vy / speed) * current_max_speed; }
        self.x += self.vx; self.y += self.vy;
        self.ax = 0.0; self.ay = 0.0;
    }
    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save(); ctx.translate(self.x, self.y).unwrap(); ctx.rotate(self.vy.atan2(self.vx)).unwrap();
        ctx.set_fill_style(&JsValue::from_str(&self.color));
        ctx.begin_path(); ctx.move_to(-self.size * 0.9, 0.0); ctx.line_to(-self.size * 1.5, -self.size * 0.6); ctx.line_to(-self.size * 1.4, 0.0); ctx.line_to(-self.size * 1.5, self.size * 0.6); ctx.close_path(); ctx.fill();
        ctx.begin_path(); ctx.ellipse(0.0, 0.0, self.size, self.size * 0.6, 0.0, 0.0, std::f64::consts::PI * 2.0).unwrap(); ctx.fill();
        ctx.set_fill_style(&JsValue::from_str("white")); ctx.begin_path(); ctx.arc(self.size * 0.6, 0.0, self.size * 0.15, 0.0, std::f64::consts::PI * 2.0).unwrap(); ctx.fill();
        ctx.set_fill_style(&JsValue::from_str("black")); ctx.begin_path(); ctx.arc(self.size * 0.65, 0.0, self.size * 0.08, 0.0, std::f64::consts::PI * 2.0).unwrap(); ctx.fill();
        ctx.restore();
    }
}

// --- Main Aquarium System ---
#[wasm_bindgen]
pub struct Aquarium {
    ctx: CanvasRenderingContext2d, width: f64, height: f64,
    god_rays: Vec<GodRay>, fishes: Vec<Fish>, food: Vec<Food>, crabs: Vec<Crab>,
    bubbles: Vec<Bubble>, frame_count: f64, castle_base_y: f64,
    castle_seed: u64,
}
#[wasm_bindgen]
impl Aquarium {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<Aquarium, JsValue> {
        // Set up panic hook for better error messages in the console.
        console_error_panic_hook::set_once();
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).ok_or_else(|| JsValue::from_str("Canvas element not found"))?.dyn_into::<HtmlCanvasElement>()?;
        canvas.set_width(web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32);
        let ctx = canvas.get_context("2d")?.ok_or_else(|| JsValue::from_str("Could not get 2d context"))?.dyn_into::<CanvasRenderingContext2d>()?;
        let width = canvas.width() as f64; let height = canvas.height() as f64;
        let god_rays = (0..(width / 200.0).clamp(3.0, 20.0).round() as usize).map(|_| GodRay::new(width, height)).collect();
        let fishes = (0..15).map(|_| Fish::new(width, height)).collect();
        let crabs = (0..3).map(|_| Crab::new(width, height)).collect();
        let castle_base_y = height * 0.95;
        let castle_center_x = width * 0.5;
        let castle_scale = (height / 1000.0).max(0.5) * 1.5;
        let bubbles = (0..30).map(|_| Bubble::new(castle_center_x, castle_base_y, castle_scale)).collect();
        let castle_seed: u64 = rand::thread_rng().next_u64();
        Ok(Self { ctx, width, height, god_rays, fishes, food: Vec::new(), crabs, bubbles, frame_count: 0.0, castle_base_y, castle_seed })
    }
    
    pub fn get_castle_scale(&self) -> f64 { (self.height / 1000.0).max(0.5) * 1.5 }
    pub fn add_food(&mut self, x: f64, y: f64) { self.food.push(Food { x, y, vy: 0.0 }); }
    pub fn add_fish(&mut self, x: f64, y: f64) { let mut fish = Fish::new(self.width, self.height); fish.x = x; fish.y = y; self.fishes.push(fish); }
    pub fn get_seafloor_y_at(&self, x: f64) -> f64 { let base_height = self.height * 0.9; let wave1 = (x * 0.005 + self.frame_count * 0.01).sin() * 10.0; let wave2 = (x * 0.02 + self.frame_count * 0.005).sin() * 5.0; base_height + wave1 + wave2 }

	fn draw_cobblestone(&self, ctx: &CanvasRenderingContext2d, path_def: impl Fn(&CanvasRenderingContext2d)) {
        let colors = ["#6c757d", "#60686f", "#788088"];
        let mut rng = StdRng::seed_from_u64(self.castle_seed);
        ctx.save();
        path_def(ctx); ctx.stroke();
        ctx.clip();
        let (stone_h, stone_w_min, stone_w_max) = (10.0, 15.0, 25.0);
        let mut y = -200.0; while y < 10.0 {
            let mut x = -150.0; 
            x += (y as i32 % 20) as f64;
            while x < 150.0 {
                let stone_w = rng.gen_range(stone_w_min..stone_w_max);
                ctx.set_fill_style(&JsValue::from_str(colors.choose(&mut rng).unwrap()));
                ctx.fill_rect(x + rng.gen_range(-1.0..1.0), y + rng.gen_range(-1.0..1.0), stone_w, stone_h);
                ctx.stroke_rect(x + rng.gen_range(-1.0..1.0), y + rng.gen_range(-1.0..1.0), stone_w, stone_h);
                x += stone_w;
            }
            y += stone_h;
        }
        ctx.restore();
    }

    fn draw_castle(&self) {
        let ctx = &self.ctx;
        ctx.save();
        ctx.translate(self.width * 0.5, self.castle_base_y);
        ctx.scale(self.get_castle_scale(), self.get_castle_scale()).unwrap();
        ctx.set_stroke_style(&JsValue::from_str("#212529")); ctx.set_line_width(2.0);
        self.draw_cobblestone(ctx, |ctx| { ctx.begin_path(); ctx.rect(-140.0, -60.0, 40.0, 60.0); });
        self.draw_cobblestone(ctx, |ctx| { ctx.begin_path(); ctx.rect(10.0, -130.0, 60.0, 130.0); });
        self.draw_cobblestone(ctx, |ctx| { ctx.begin_path(); ctx.rect(70.0, -100.0, 50.0, 100.0); });
        self.draw_cobblestone(ctx, |ctx| {
            ctx.begin_path(); ctx.move_to(-100.0, 0.0); ctx.line_to(-100.0, -80.0); ctx.line_to(-50.0, -90.0); ctx.line_to(10.0, -85.0); ctx.line_to(10.0, 0.0); ctx.close_path();
        });
        self.draw_cobblestone(ctx, |ctx| {
            ctx.begin_path(); ctx.move_to(-50.0, -90.0); ctx.line_to(-50.0, -110.0); ctx.line_to(-40.0, -125.0); ctx.line_to(-30.0, -115.0); ctx.line_to(-20.0, -120.0); ctx.line_to(-20.0, -88.0); ctx.close_path();
        });
        let draw_crenellations = |ctx: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, n: f64| {
            let merlon_w = w / n; let merlon_h = merlon_w * 0.8;
            for i in 0..n as usize { if i % 2 == 0 { ctx.begin_path(); ctx.rect(x + i as f64 * merlon_w, y - merlon_h, merlon_w, merlon_h); ctx.set_fill_style(&JsValue::from_str("#6c757d")); ctx.fill(); ctx.stroke(); } }
        };
        let draw_arched_opening = |ctx: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64| {
            ctx.save(); ctx.set_fill_style(&JsValue::from_str("#212529"));
            ctx.begin_path(); ctx.move_to(x, y); ctx.line_to(x, y - h); ctx.arc(x + w / 2.0, y - h, w / 2.0, std::f64::consts::PI, 0.0).unwrap(); ctx.line_to(x + w, y); ctx.close_path(); ctx.fill();
            ctx.restore();
        };
        draw_crenellations(ctx, -140.0, -60.0, 40.0, 4.0);
        draw_crenellations(ctx, -90.0, -85.0, 40.0, 4.0);
        draw_arched_opening(ctx, -90.0, -20.0, 25.0, 30.0);
        draw_crenellations(ctx, 10.0, -130.0, 60.0, 5.0);
        draw_crenellations(ctx, 70.0, -100.0, 50.0, 4.0);
        draw_arched_opening(ctx, 80.0, -10.0, 30.0, 40.0);
        ctx.set_fill_style(&JsValue::from_str("#212529"));
        ctx.begin_path(); ctx.rect(-40.0, -50.0, 80.0, 50.0); ctx.fill();
        ctx.begin_path(); ctx.arc(0.0, -50.0, 40.0, std::f64::consts::PI, 0.0).unwrap(); ctx.fill();
        ctx.restore();
    }

    pub fn tick(&mut self) {
        self.frame_count += 1.0;
        self.draw_background();
        self.draw_castle();
        self.draw_seafloor();
        self.update_bubbles();
        self.update_crabs();
        self.update_fishes();
        self.update_food();
    }
    
    fn draw_background(&mut self) { 
        let bg_gradient = self.ctx.create_linear_gradient(0.0, 0.0, 0.0, self.height);
        bg_gradient.add_color_stop(0.0, "#005c97").unwrap(); bg_gradient.add_color_stop(1.0, "#06223b").unwrap();
        self.ctx.set_fill_style_canvas_gradient(&bg_gradient); self.ctx.fill_rect(0.0, 0.0, self.width, self.height);
        for ray in &mut self.god_rays { ray.update(); if ray.life >= ray.max_life { ray.reset(self.width, self.height); } ray.draw(&self.ctx); }
    }
    fn draw_seafloor(&self) { 
        self.ctx.begin_path(); self.ctx.move_to(0.0, self.height * 0.9);
        let mut x = 0.0; while x < self.width + 10.0 { self.ctx.line_to(x, self.get_seafloor_y_at(x)); x += 10.0; }
        self.ctx.line_to(self.width, self.height); self.ctx.line_to(0.0, self.height); self.ctx.close_path();
        self.ctx.set_fill_style(&JsValue::from_str("#c2b280")); self.ctx.fill();
    }
    fn update_bubbles(&mut self) {
        let castle_center_x = self.width * 0.5;
        let castle_scale = self.get_castle_scale();
        for bubble in &mut self.bubbles {
            bubble.update();
            if bubble.y < -bubble.size { bubble.reset(castle_center_x, self.castle_base_y, castle_scale); }
            bubble.draw(&self.ctx);
        }
    }
    fn update_food(&mut self) {
        let food_floor_ys: Vec<f64> = self.food.iter().map(|f| self.get_seafloor_y_at(f.x)).collect();
        for (i, food_item) in self.food.iter_mut().enumerate() {
            let floor_y = food_floor_ys[i];
            if food_item.y < floor_y { food_item.vy += 0.007; food_item.vy *= 0.99; food_item.y += food_item.vy; }
            if food_item.y > floor_y { food_item.y = floor_y; }
            self.ctx.set_fill_style(&JsValue::from_str("#f0e68c"));
            self.ctx.begin_path(); self.ctx.arc(food_item.x, food_item.y, 3.0, 0.0, std::f64::consts::PI * 2.0).unwrap(); self.ctx.fill();
        }
    }
    fn update_crabs(&mut self) {
        let crab_floor_ys: Vec<f64> = self.crabs.iter().map(|c| self.get_seafloor_y_at(c.x)).collect();
        for (i, crab) in self.crabs.iter_mut().enumerate() { crab.update(crab_floor_ys[i], self.width); crab.draw(&self.ctx); }
    }
    fn update_fishes(&mut self) {
        let mut food_to_remove = Vec::new();
        let fish_floor_ys: Vec<f64> = self.fishes.iter().map(|f| self.get_seafloor_y_at(f.x)).collect();
        for (i, fish) in self.fishes.iter_mut().enumerate() {
            fish.update(&self.food, fish_floor_ys[i], self.width, self.height);
            if let Some((dist_sq, food_idx)) = self.food.iter().enumerate().map(|(i, f)| ((fish.x - f.x).powi(2) + (fish.y - f.y).powi(2), i)).min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()) {
                if dist_sq < (fish.size + 3.0).powi(2) { food_to_remove.push(food_idx); }
            }
            fish.draw(&self.ctx);
        }
        food_to_remove.sort_unstable(); food_to_remove.dedup();
        for &index in food_to_remove.iter().rev() { if index < self.food.len() { self.food.remove(index); } }
    }
}