extern crate rand;
extern crate sdl2;

use std::f32;

use rand::Rng;
use rand::rngs::ThreadRng;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod vector;

use vector::Vec2d;

const LUT_RESOLUTION: usize = 1 << 9;
const NO_OF_STARS: u32 = 512;
const SCREEN_WIDTH: u32 = 512;
const SCREEN_HEIGHT: u32 = 512;

#[derive(Debug)]
struct Star {
    position: Vec2d,
    velocity: Vec2d,
    color: u8,
}

impl Star {
    fn randomize(&mut self, rng: &mut ThreadRng) {
        self.position.x = rng.gen::<f32>() * SCREEN_WIDTH as f32 + SCREEN_WIDTH as f32;
        self.position.y = rng.gen::<f32>() * SCREEN_HEIGHT as f32;
        self.velocity.x = -rng.gen::<f32>();
        self.color = (-self.velocity.x * 255.0) as u8;
    }

    fn new() -> Star {
        Star {
            position: Vec2d { x: 0.0, y: 0.0 },
            velocity: Vec2d { x: 0.0, y: 0.0 },
            color: 0,
        }
    }
}

fn generate_stars(rng: &mut ThreadRng) -> Vec<Star> {
    let mut stars = Vec::new();
    for _ in 0..NO_OF_STARS {
        let mut star = Star::new();
        star.randomize(rng);
        stars.push(star);
    }

    stars
}

fn generate_sin_lut() -> Vec<f32> {
    let mut lut: Vec<f32> = vec![0.0; LUT_RESOLUTION];
    let step: f32 = (f32::consts::PI * 2.0) / LUT_RESOLUTION as f32;
    let mut a: f32 = 0.0;
    for i in 0..LUT_RESOLUTION {
        lut[i] = a.sin();
        a += step;
    }

    lut
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("J-25", SCREEN_WIDTH, SCREEN_HEIGHT)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut rng = rand::thread_rng();
    let sin_lut = generate_sin_lut();
    let mut stars = generate_stars(&mut rng);

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'mainloop,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for star in &mut stars {
            let offset = sin_lut[(star.position.x as usize) & (LUT_RESOLUTION - 1)] * star.velocity.x * 75.0;
            let y = star.position.y + offset;

            canvas.set_draw_color(Color::RGB(star.color, star.color, star.color));
            let _ = canvas.draw_point(Point::new(star.position.x as i32, y as i32));

            star.position = star.position + star.velocity;
            if star.position.x < 0.0 {
                star.randomize(&mut rng);
            }
        }

        canvas.present();
    }
}
