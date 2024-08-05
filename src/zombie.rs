use std::time::Duration;

pub struct Zombie {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

impl Zombie {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, speed: 50.0 }
    }

    pub fn update(&mut self, delta_time: Duration, player_x: f32, player_y: f32) {
        let delta_seconds = delta_time.as_secs_f32();
        let dx = player_x - self.x;
        let dy = player_y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 0.0 {
            self.x += dx / distance * self.speed * delta_seconds;
            self.y += dy / distance * self.speed * delta_seconds;
        }
    }
}

pub trait ZombieTrait {
    fn get_position(&self) -> (f32, f32);
}

impl ZombieTrait for Zombie {
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
