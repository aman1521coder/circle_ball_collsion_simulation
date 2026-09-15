// circle ball collision using rust 

use macroquad::prelude::*;

struct Ball {
    radius: f32, // Fixed typo from 'raduis'
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}
struct   Big_Circle{
    radius: f32,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}
impl  Big_Circle {
    fn new(radius: f32, x: f32, y: f32, vx: f32, vy: f32) -> Big_Circle {
        Big_Circle {
            radius,
            x,
            y,
            vx,
            vy,
        }
    }
    fn move_circle_right(&mut self) {
        self.x += self.vx;
    }
    fn move_circle_left(&mut self) {
        self.x -= self.vx;
    }
    fn move_circle_up(&mut self){
        self.y-=self.vx
    }
    fn move_circle_down(&mut self){
        self.y+=self.vx
    }
    fn border_check(&mut self){
        let width=macroquad::window::screen_width();
        let height=macroquad::window::screen_height();
        if self.x+self.radius>width{
            self.x=width-self.radius;
        }
        if self.x-self.radius<0.0{
            self.x=self.radius;
        }
        if self.y+self.radius>height{
            self.y=height-self.radius;
        }
        if self.y-self.radius<0.0{
            self.y=self.radius;
        }
    }

}

impl Ball {
    fn new(radius: f32, x: f32, y: f32, vx: f32, vy: f32) -> Ball {
        Ball {
            radius,
            x,
            y,
            vx,
            vy,
        }
    }
}


impl Ball {

   

   fn check_collision(&mut self, circle: &Big_Circle) {
    let gravity = 500.0;
    let dt = get_frame_time();

    // Gravity
    self.vy += gravity * dt;

    // Move ball
    self.x += self.vx * dt;
    self.y += self.vy * dt;

    // Position relative to moving circle
    let dx = self.x - circle.x;
    let dy = self.y - circle.y;

    let distance = (dx * dx + dy * dy).sqrt();

    let allowed_distance = circle.radius - self.radius;

    if distance > allowed_distance {
        // Normal pointing from circle center -> ball
        let nx = dx / distance;
        let ny = dy / distance;

        // Push ball back inside
        let penetration = distance - allowed_distance;

        self.x -= nx * penetration;
        self.y -= ny * penetration;

        // -------------------------
        // Relative velocity
        // -------------------------

        let relative_vx = self.vx - circle.vx;
        let relative_vy = self.vy - circle.vy;

        // Velocity along collision normal
        let dot = relative_vx * nx + relative_vy * ny;

        let restitution = 0.8;

        // Bounce relative velocity
        let new_relative_vx =
            relative_vx - (1.0 + restitution) * dot * nx;

        let new_relative_vy =
            relative_vy - (1.0 + restitution) * dot * ny;

        // Convert back to world velocity
        self.vx = new_relative_vx + circle.vx;
        self.vy = new_relative_vy + circle.vy;
    }
}
    }


#[macroquad::main("Circle Collision")]
async fn main() {
    // Note: vx and vy act as the continuous speed *and* the manual keyboard push force
    let mut ball = Ball::new(10.0, 400.0, 300.0, 4.0, 4.0);
    let mut big_circle=Big_Circle::new(200.0,400.0,300.0,4.0,4.0);

    loop {
        clear_background(BLACK);

        // Draw the outer container ring
        draw_circle_lines(big_circle.x, big_circle.y, big_circle.radius, 3.0, WHITE);
        
        // Draw the player ball
        draw_circle(ball.x, ball.y, ball.radius, WHITE);

        // Handle keyboard movement overrides
        if is_key_down(KeyCode::Up) {
            big_circle.move_circle_up();
        }
        if is_key_down(KeyCode::Down) {
            big_circle.move_circle_down();
        }
        if is_key_down(KeyCode::Left) {
            big_circle.move_circle_left();
        }   
        if is_key_down(KeyCode::Right) {
          big_circle.move_circle_right();
        }

        // Run the physics check and update bounces automatically
        ball.check_collision(&big_circle);
        big_circle.border_check();

        next_frame().await;
    }
}
