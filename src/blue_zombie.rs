use std::time::Duration;
use crate::zombie::ZombieTrait;

pub struct BlueZombie {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

impl BlueZombie {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, speed: 85.0 }
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

impl ZombieTrait for BlueZombie {
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
