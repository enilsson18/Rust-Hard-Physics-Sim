extern crate sdl2;
extern crate vecmath;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
//use std::time::Duration;
use sdl2::rect::{Point, Rect};

struct RenderRect<'a> {
    pos: vecmath::Vector2<i32>,
    size: vecmath::Vector2<u32>,
    angle: f64,
    color: Color,
    texture: sdl2::render::Texture<'a>,
}

impl<'a> RenderRect<'a>{
    pub fn new(
        pos: vecmath::Vector2<i32>, 
        size: vecmath::Vector2<u32>,
        angle: f64,
        color: Color,
        creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext
        >) -> Result<RenderRect, String> {

        let mut texture = creator
            .create_texture_target(PixelFormatEnum::RGBA8888, size[0], size[1])
            .map_err(|e| e.to_string())?;

        let rect = RenderRect {
            pos: pos,
            size: size,
            angle: angle,
            color: color,
            texture: texture,
        };

        Ok(rect)
    }

    pub fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        render_rect(self.color, self.angle, &mut self.texture, canvas, self.pos, self.size);
    }
}

struct PhysicsRect {
    pos: vecmath::Vector2<f32>,
    size: vecmath::Vector2<f32>,
    velocity: vecmath::Vector2<f32>,
    angle: f64,
    angularVelocity: vecmath::Vector2<f32>,
    mass: f32,
}

fn syncRPRect(render: &mut RenderRect, physics: &PhysicsRect) {
    render.pos = [physics.pos[0] as i32, physics.pos[1] as i32];
    render.size = [physics.size[0] as u32, physics.size[1] as u32];
    render.angle = physics.angle;
}

// apply the screen bound constraint on velocity
fn screenBoundConstraint(rect: &mut PhysicsRect) {
    // apply for each corner of the square
    // just the bottom left is being processed for now
    let newPos = vecmath::vec2_add(rect.pos, rect.velocity);

    let corners = [[rect.pos[0], rect.pos[1] + rect.size[1]]];

    // find How much they are overlapping
    let normal = [0.0, 1.0];
    let radius = [0.5, 0.5];

    let error = vecmath::vec2_len(vecmath::vec2_scale(normal, 500.0 - corners[0][1]));

    if error < 0.0 {
        let dError = vecmath::vec2_mul(rect.velocity, normal);

        vecmath::vec2_sub(rect.velocity, vecmath::vec2_scale(normal, error));
    }
}

fn render_rect<'a> (color: Color, angle: f64, texture: &mut sdl2::render::Texture<'a>, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, pos: vecmath::Vector2<i32>, size: vecmath::Vector2<u32>) {
    canvas
        .with_texture_canvas(texture, |texture_canvas| {
            texture_canvas.clear();
            texture_canvas.set_draw_color(color);
            texture_canvas
                .fill_rect(Rect::new(pos[0], pos[1], size[0], size[1]))
                .expect("could not fill rect");
        })
        .map_err(|e| e.to_string());

    //println!("{} {} {} {}", pos[0], pos[1], size[0], size[1]);

        
    let dst = Some(Rect::new(pos[0], pos[1], size[0], size[1]));
    canvas.copy_ex(
        &texture,
        None,
        dst,
        angle,
        Some(Point::new(pos[0] + (size[0] as i32) / 2, pos[1] + (size[1] as i32) / 2)),
        false,
        false,
    );
    
}

fn clearScreen(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-sdl2 resource-manager demo", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();

    let mut rect = RenderRect::new([300, 200], [200, 200], 0.0, Color::RGBA(255, 0, 0, 255), &creator).unwrap();
    let mut pRect = PhysicsRect {
        pos: [300.0, 200.0],
        size: [200.0, 200.0],
        velocity: [0.0, 0.0],
        angle: 0.0,
        angularVelocity: [0.0, 0.0],
        mass: 1.0,
    };

    let dt = 0.001;

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
        //rect.angle = (rect.angle + 0.5) % 360.;

        // apply gravity
        //vecmath::vec2_add(pRect.velocity, [0.0, 9.8 * dt]);

        // constrain velocity
        //screenBoundConstraint(&mut pRect);

        // sync rects
        //syncRPRect(&mut rect, &pRect);

        clearScreen(&mut canvas);
        
        rect.render(&mut canvas);

        canvas.present();

        // Normalize the loop speed (Count how long has passed for computation and then sleep the rest of the time per frame)
        // Delay Creation tool
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

    }

    Ok(())
}