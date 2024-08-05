use rand::Rng;
use std::time::{Duration, Instant};
use crate::zombie::Zombie;
use crate::blue_zombie::BlueZombie;

pub struct ZombieSpawner {
    pub last_spawn: Instant,
    pub spawn_interval: Duration,
    pub spawn_rate_decrease: Duration,
    pub min_spawn_interval: Duration,
}

impl ZombieSpawner {
    pub fn new(spawn_interval: Duration, spawn_rate_decrease: Duration, min_spawn_interval: Duration) -> Self {
        Self {
            last_spawn: Instant::now(),
            spawn_interval,
            spawn_rate_decrease,
            min_spawn_interval,
        }
    }

    pub fn spawn_zombies(&mut self, zombies: &mut Vec<Zombie>, blue_zombies: &mut Vec<BlueZombie>) {
        if self.last_spawn.elapsed() >= self.spawn_interval {
            let mut rng = rand::thread_rng();
            let edge = rng.gen_range(0..4); // Randomly select one of the four edges

            let (x, y) = match edge {
                0 => (rng.gen_range(0.0..800.0), 0.0), // Top edge
                1 => (rng.gen_range(0.0..800.0), 600.0), // Bottom edge
                2 => (0.0, rng.gen_range(0.0..600.0)), // Left edge
                3 => (800.0, rng.gen_range(0.0..600.0)), // Right edge
                _ => (400.0, 300.0), // Center (shouldn't happen)
            };

            if rng.gen_bool(0.2) { // 20% chance to spawn a BlueZombie
                blue_zombies.push(BlueZombie::new(x, y));
            } else {
                zombies.push(Zombie::new(x, y));
            }
            
            self.last_spawn = Instant::now();

            // Decrease the spawn interval more aggressively
            if self.spawn_interval > self.min_spawn_interval {
                self.spawn_interval = self.spawn_interval.saturating_sub(self.spawn_rate_decrease);
                if self.spawn_interval < self.min_spawn_interval {
                    self.spawn_interval = self.min_spawn_interval;
                }
            }
        }
    }
}
