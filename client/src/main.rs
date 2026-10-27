mod graphics;
mod entity;

use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use hecs::World;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::PixelFormat;
use sdl3::render::ScaleMode;
use crate::entity::{PositionBundle, VelocityBundle};
use crate::graphics::pixel_buffer::PixelBuffer;
use crate::graphics::Renderer;

fn main() -> anyhow::Result<()> {
    // let sdl3 = sdl3::init()?;
    //
    // let video = sdl3.video()?;
    // let window = video.window(format!("netcraft: v{}", env!("CARGO_PKG_VERSION")).as_str(), 1280, 720).position_centered().build()?;
    // let mut event_pump = sdl3.event_pump()?;
    // let mut renderer = Renderer::new(window.into_canvas())?;
    //
    // let mut world = World::new();
    //
    // world.spawn((PositionBundle {
    //     x: 0.0,
    //     y: 0.0,
    // }, VelocityBundle {
    //     x: 0.0,
    //     y: 0.0,
    // }));

    let server_addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let mut stream = TcpStream::connect(server_addr).unwrap();

    loop {
        // for vel in world.query_mut::<&mut VelocityBundle>() {
        //     vel.y = 0.0;
        //     vel.x = 0.0;
        // }
        //
        // for event in event_pump.poll_iter() {
        //     match event {
        //         Event::Quit { .. } => return Ok(()),
        //         Event::KeyDown { keycode: Some(key), .. } => {
        //             for vel in world.query_mut::<&mut VelocityBundle>() {
        //                 if let Keycode::W = key {
        //                     vel.y = -1.0;
        //                 }
        //
        //                 if let Keycode::S = key {
        //                     vel.y += 1.0;
        //                 }
        //
        //                 if let Keycode::A = key {
        //                     vel.x = -1.0;
        //                 }
        //
        //                 if let Keycode::D = key {
        //                     vel.x += 1.0;
        //                 }
        //             }
        //         }
        //         _ => {},
        //     }
        // }
        //
        // for (pos, vel) in world.query_mut::<(&mut PositionBundle, &VelocityBundle)>() {
        //     pos.x += vel.x;
        //     pos.y += vel.y;
        // }
        //
        // renderer.flush();
        //
        // for pos in &mut world.query::<&PositionBundle>() {
        //     renderer.blit_pixel(pos.x as i32, pos.y as i32, 0xFF00FFFF);
        // }
        //
        // renderer.splat()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        stream.write(input.as_bytes())?;
    }
}
