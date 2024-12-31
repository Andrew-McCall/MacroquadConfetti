use std::u8;

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    id: usize,
    x: f32,
    y: f32,
    x_vel: f32,
    y_vel: f32,
    color: Color,
}

static mut BALL_ID : usize = 0;
const GRAVITY: f32 = 1.0;
const BALL_RADIUS: f32 = 6.0;
const FLOOR_LOSS: f32 = 0.8;
const DRAG: f32 = 0.95;
const BALL_RADIUS_SQR: f32= BALL_RADIUS * BALL_RADIUS;

impl Ball {
    fn new(x: f32, y:f32, seed:usize) -> Ball{   
        let id;
        unsafe {
            id = BALL_ID;
            BALL_ID += 1;
        }

        let seed = seed % (usize::MAX/64);
        let random = (32_1239 * seed*seed*seed* 17 + id)% 1000 ;
        let x_vel = random % 9;
        let x_vel =  (x_vel as f32 - 4.0) / 4.0;

        return Ball{id, x, y, x_vel, y_vel:0.0, color:Color::from_rgba((random as usize*47%255) as u8, (random as usize*29%255) as u8, (random as usize*101%255) as u8, u8::MAX)}
    }

    fn tick(&mut self){
        self.x_vel *= DRAG;
        self.y_vel *= DRAG;

        self.x += self.x_vel;
        self.y += self.y_vel;

        if self.y > screen_height(){
            // self.y = screen_height();
            self.y_vel = -self.y_vel * FLOOR_LOSS;
        }else{
            self.y_vel += GRAVITY;
        }

        if self.x < 0.0 || self.x > screen_width(){
            self.x_vel = -self.x_vel * FLOOR_LOSS;
        }
    }

    fn draw(&self){
        draw_circle(self.x, self.y-BALL_RADIUS, BALL_RADIUS, self.color);
    }

    fn collide(&mut self, balls: &[Ball]) {
        for other in balls {
            if other.id == self.id {
                continue; 
            }

            let dx = self.x - other.x;
            let dy = self.y - other.y;
            let distance_sqr = dx * dx + dy * dy;

            if distance_sqr > BALL_RADIUS_SQR {
                continue; 
            }

            let distance = distance_sqr.sqrt();
            if distance == 0.0 {
                continue;
            }
            let nx = dx / distance; 
            let ny = dy / distance; 

            let relative_velocity_x = self.x_vel - other.x_vel;
            let relative_velocity_y = self.y_vel - other.y_vel;
            let relative_velocity_normal = relative_velocity_x * nx + relative_velocity_y * ny;

            if relative_velocity_normal > 0.0 {
                continue;
            }

            let impulse = (2.0 * relative_velocity_normal) / 2.0 * FLOOR_LOSS;

            self.x_vel -= impulse * nx;
            self.y_vel -= impulse * ny;
        }
    }

}

#[macroquad::main("AM - Confetti")]
async fn main() {

    let mut balls: Vec<Ball> = Vec::new();
    let mut frame_count: usize = 0;
    loop {
        // Loop start
        frame_count += 1;
        clear_background(Color { r: 0.95, g: 0.9, b: 0.9, a: 1.0 });

        // Handle Inputs
        if is_mouse_button_down(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Right) {
            let (mouse_x,mouse_y) = mouse_position();
            balls.push(Ball::new(mouse_x, mouse_y ,frame_count))
        }

        // Handle Tick
        let balls_prev = balls.to_vec();        
        for ball in &mut balls {
            ball.collide(&balls_prev);
            ball.tick();
            ball.draw();
        }   

        unsafe {
            draw_text(&format!("Balls: {}", BALL_ID), 15.0, 25.0, 32.0, BLACK);
        }

        next_frame().await
    }
}