// circle ball collision using rust 

use macroquad::prelude::*;

struct Ball {
    radius: f32, // Fixed typo from 'raduis'
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
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
    fn move_ball_right(&mut self) {
        self.x += self.vx;
    }
    fn move_ball_left(&mut self) {
        self.x -= self.vx;
    }   
    fn move_ball_down(&mut self) {
        self.y += self.vy;
    }
    fn move_ball_up(&mut self) {
        self.y -= self.vy;
    }

    // Checks collision against the outer circle ring boundary
    fn check_collision(&mut self)->(f32,f32) {
        let center_x = 400.0;
        let center_y = 300.0;
        let container_radius = 200.0;
               let gravity=500.0;
               let damping=0.5;
            self.vy+=gravity*get_frame_time();
            self.x += self.vx*get_frame_time();
            self.y += self.vy*get_frame_time();
        let dx = self.x - center_x;
        let dy = self.y - center_y;
        let distance = (dx.powf(2.0) + dy.powf(2.0)).sqrt();

        // Collision happens when the ball's outer edge passes the container's inner edge
        if distance + self.radius > container_radius {
            // 1. Calculate the surface normal vector pointing inward
            let nx = dx / distance;
            let ny = dy / distance;
           let dot =nx*self.vx + ny*self.vy;
           let new_vx= self.vx - 2.0 * dot * nx;
           let new_vy= self.vy - 2.0 * dot * ny;
           self.vx=new_vx;
           self.vy=new_vy;
            // 2. Push the ball back inside so it doesn't get stuck in the wall
            let allowed_distance=180.0;
            let pen=distance-allowed_distance;
            self.x -= pen * nx;
            self.y -= pen * ny;
      
            (nx,ny)
        }else {
            (0.0,0.0)
        }

    }
}

#[macroquad::main("Circle Collision")]
async fn main() {
    // Note: vx and vy act as the continuous speed *and* the manual keyboard push force
    let mut ball = Ball::new(20.0, 400.0, 300.0, 4.0, 4.0);

    loop {
        clear_background(BLACK);

        // Draw the outer container ring
        draw_circle_lines(400.0, 300.0, 200.0, 3.0, WHITE);
        
        // Draw the player ball
        draw_circle(ball.x, ball.y, ball.radius, BLUE);

        // Handle keyboard movement overrides
        if is_key_down(KeyCode::Up) {
            ball.move_ball_up();
        }
        if is_key_down(KeyCode::Down) {
            ball.move_ball_down();
        }
        if is_key_down(KeyCode::Left) {
            ball.move_ball_left();
        }   
        if is_key_down(KeyCode::Right) {
            ball.move_ball_right();
        }

        // Run the physics check and update bounces automatically
        ball.check_collision();

        next_frame().await;
    }
}
