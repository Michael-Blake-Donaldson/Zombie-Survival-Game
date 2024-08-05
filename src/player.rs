use std::time::Duration;
use winit::event::VirtualKeyCode;
use crate::zombie::ZombieTrait;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub health: i32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, speed: 100.0, health: 100 }
    }

    pub fn update(&mut self, delta_time: Duration, keys_pressed: &Vec<VirtualKeyCode>) {
        let delta_seconds = delta_time.as_secs_f32();
        for key in keys_pressed {
            match key {
                VirtualKeyCode::W => self.y -= self.speed * delta_seconds,
                VirtualKeyCode::S => self.y += self.speed * delta_seconds,
                VirtualKeyCode::A => self.x -= self.speed * delta_seconds,
                VirtualKeyCode::D => self.x += self.speed * delta_seconds,
                _ => (),
            }
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn collides_with<T: ZombieTrait>(&self, zombie: &T) -> bool {
        let (zombie_x, zombie_y) = zombie.get_position();
        let dx = self.x - zombie_x;
        let dy = self.y - zombie_y;
        (dx * dx + dy * dy).sqrt() < 5.0
    }
}
