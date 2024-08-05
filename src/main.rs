use std::time::{Duration, Instant};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent, KeyboardInput},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use rusttype::{Font, Scale};

mod player;
mod zombie;
mod blue_zombie;
mod zombie_spawner;
use player::Player;
use zombie::Zombie;
use blue_zombie::BlueZombie;
use zombie_spawner::ZombieSpawner;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

enum GameState {
    Playing,
    GameOver,
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Zombie Survival Game")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

    // Load the font
    let font_data: &[u8] = include_bytes!("../resources/Aladin-Regular.ttf");
    let font = Font::try_from_bytes(font_data).expect("Error loading font");

    let mut player = Player::new(400.0, 300.0); // Center player
    let mut zombies = vec![
        Zombie::new(100.0, 100.0),
        Zombie::new(700.0, 500.0),
        Zombie::new(200.0, 400.0),
    ];
    let mut blue_zombies: Vec<BlueZombie> = vec![];
    let mut keys_pressed = Vec::new();
    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(16); // ~60 FPS
    let mut game_state = GameState::Playing;
    let mut score = 0;
    let mut score_update = Instant::now();
    let mut zombie_spawner = ZombieSpawner::new(Duration::from_secs(5), Duration::from_millis(100), Duration::from_secs(1)); // Spawn a new zombie every 5 seconds, decreasing by 100ms each time, to a minimum of 1 second

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => {
                    match state {
                        ElementState::Pressed => {
                            if !keys_pressed.contains(&keycode) {
                                keys_pressed.push(keycode);
                            }
                        }
                        ElementState::Released => {
                            keys_pressed.retain(|&k| k != keycode);
                        }
                    }
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let now = Instant::now();
                let delta_time = now - last_update;

                if delta_time >= update_interval {
                    if let GameState::Playing = game_state {
                        player.update(delta_time, &keys_pressed);
                        for zombie in zombies.iter_mut() {
                            zombie.update(delta_time, player.x, player.y);
                        }
                        for blue_zombie in blue_zombies.iter_mut() {
                            blue_zombie.update(delta_time, player.x, player.y);
                        }

                        for zombie in &zombies {
                            if player.collides_with(zombie) {
                                player.take_damage(10);
                                println!("Player collided with a zombie! Health: {}", player.health);
                            }
                        }
                        for blue_zombie in &blue_zombies {
                            if player.collides_with(blue_zombie) {
                                player.take_damage(10);
                                println!("Player collided with a blue zombie! Health: {}", player.health);
                            }
                        }

                        if player.health <= 0 {
                            game_state = GameState::GameOver;
                        }

                        // Update score every second
                        if score_update.elapsed() >= Duration::from_secs(1) {
                            score += 1;
                            score_update = Instant::now();
                        }

                        // Spawn new zombies
                        zombie_spawner.spawn_zombies(&mut zombies, &mut blue_zombies);
                    }

                    last_update = now;

                    // Redraw the screen
                    pixels.get_frame().fill(0); // Clear the screen
                    if let GameState::Playing = game_state {
                        draw_player(pixels.get_frame(), &player);
                        for zombie in &zombies {
                            draw_zombie(pixels.get_frame(), zombie);
                        }
                        for blue_zombie in &blue_zombies {
                            draw_blue_zombie(pixels.get_frame(), blue_zombie);
                        }
                        draw_text(pixels.get_frame(), &font, &format!("Health: {}", player.health), 10.0, 10.0);
                        draw_text(pixels.get_frame(), &font, &format!("Score: {}", score), 10.0, 40.0);
                    } else {
                        draw_text(pixels.get_frame(), &font, "Game Over", WIDTH as f32 / 2.0 - 60.0, HEIGHT as f32 / 2.0);
                        draw_text(pixels.get_frame(), &font, &format!("Score: {}", score), WIDTH as f32 / 2.0 - 60.0, HEIGHT as f32 / 2.0 + 40.0);
                    }

                    if let Err(e) = pixels.render() {
                        eprintln!("Error rendering pixels: {}", e);
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
            _ => (),
        }
    });
}

fn draw_player(frame: &mut [u8], player: &Player) {
    draw_rectangle(frame, player.x as u32, player.y as u32, 5, 5, [0x00, 0xFF, 0x00, 0xFF]);
}

fn draw_zombie(frame: &mut [u8], zombie: &Zombie) {
    draw_rectangle(frame, zombie.x as u32, zombie.y as u32, 5, 5, [0xFF, 0x00, 0x00, 0xFF]);
}

fn draw_blue_zombie(frame: &mut [u8], blue_zombie: &BlueZombie) {
    draw_rectangle(frame, blue_zombie.x as u32, blue_zombie.y as u32, 5, 5, [0x00, 0x00, 0xFF, 0xFF]);
}

fn draw_rectangle(frame: &mut [u8], x: u32, y: u32, width: u32, height: u32, color: [u8; 4]) {
    for i in 0..width {
        for j in 0..height {
            let xi = x + i;
            let yj = y + j;
            if xi < WIDTH && yj < HEIGHT {
                let index = ((yj * WIDTH) + xi) as usize * 4;
                frame[index] = color[0]; // R
                frame[index + 1] = color[1]; // G
                frame[index + 2] = color[2]; // B
                frame[index + 3] = color[3]; // A
            }
        }
    }
}

fn draw_text(frame: &mut [u8], font: &Font, text: &str, x: f32, y: f32) {
    let scale = Scale::uniform(20.0);
    let v_metrics = font.v_metrics(scale);

    let glyphs: Vec<_> = font.layout(text, scale, rusttype::point(x, y + v_metrics.ascent)).collect();
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, v| {
                let gx = gx as i32 + bounding_box.min.x;
                let gy = gy as i32 + bounding_box.min.y;
                if gx >= 0 && gx < WIDTH as i32 && gy >= 0 && gy < HEIGHT as i32 {
                    let index = ((gy as u32 * WIDTH) + gx as u32) as usize * 4;
                    if index + 3 < frame.len() {
                        frame[index] = (v * 255.0) as u8; // R
                        frame[index + 1] = (v * 255.0) as u8; // G
                        frame[index + 2] = (v * 255.0) as u8; // B
                        frame[index + 3] = 255; // A
                    }
                }
            });
        }
    }
}
