use color_eyre::Result;
use itertools::Itertools;
use minifb::{Key, Window, WindowOptions};
use prime_tools::is_u32_prime;
mod components;
mod screen;

const WIDTH: usize = 640 * 3;
const HEIGHT: usize = 360 * 3;
const FPS: u64 = 60;

use crate::{
    components::{Blinker, Breather},
    screen::{Channel, Pixel, Pos, Screen},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Hello, world!");

    let mut window = Window::new(
        "cool",
        WIDTH,
        HEIGHT,
        WindowOptions {
            // transparency: true,
            // borderless: true,
            ..Default::default()
        },
    )?;

    window.limit_update_rate(Some(std::time::Duration::from_millis(1000 / FPS)));

    let buffer = vec![0_u32; WIDTH * HEIGHT];
    let mut screen = Screen::new(
        buffer,
        WIDTH,
        HEIGHT,
        vec![
            Box::new(Blinker::new(Pos { x: 0, y: 0 })),
            Box::new(Breather::new(
                Pixel {
                    pos: Pos { x: 1, y: 0 },
                    color: 0,
                },
                Channel::Green,
            )),
        ],
    );

    // while window.is_open() && !window.is_key_down(Key::Escape) {
    for _ in 0..(5 * FPS) {
        screen.render();
        window.update_with_buffer(&screen, WIDTH, HEIGHT).unwrap();
        screen.tick();
    }

    Ok(())
}

// for (b, (x, y)) in (*screen)
//     .iter_mut()
//     .zip((0..HEIGHT).cartesian_product(0..WIDTH))
// {
//     let x = x as isize;
//     let y = y as isize;
//     *b = if (y == tick || x == tick)
//         || [-20, -15, -10, -5, 0, 5, 10, 15, 20].into_iter().any(|o| {
//             (y == WIDTH as isize / 2 + o)
//                 || (x - 420 == HEIGHT as isize / 2 + o)
//                 || (x.saturating_add(o) == y)
//                 || ((WIDTH as isize - x).saturating_add(o) == y)
//         }) {
//         // fastrand::u32(..)

//         0xffffffff
//     } else {
//         0x00000000
//     }
// }
