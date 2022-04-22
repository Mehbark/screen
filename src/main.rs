use color_eyre::Result;
use components::Direction;
use itertools::Itertools;
use minifb::{Key, Window, WindowOptions};
use prime_tools::is_u32_prime;
mod components;
mod screen;

const WIDTH: usize = 360 * 3;
const HEIGHT: usize = 640 * 3;
const FPS: u64 = 144;

use crate::{
    components::{Blinker, Breather, FallingSand, Laser},
    screen::{Bounds, Channel, Pixel, Pos, Screen},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut window = Window::new(
        "cool - esc to close lol",
        WIDTH,
        HEIGHT,
        WindowOptions {
            transparency: true,
            borderless: true,
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
            // (Box::new(Blinker::new()), Pos { x: 0, y: 0 }),
            // (
            //     Box::new(Breather::new(0x00000000, Channel::Green)),
            //     Pos { x: 1, y: 1 },
            // ),
            (
                Box::new(Laser::new(
                    Bounds {
                        width: WIDTH,
                        height: HEIGHT,
                    },
                    Pos { x: 0, y: 0 },
                    0xfff0d9b5,
                )),
                Pos { x: 0, y: 0 },
            ), // Box::new(FallingSand::new_num_sands(100, WIDTH, HEIGHT)),
            (
                Box::new(Laser::new(
                    Bounds {
                        width: WIDTH,
                        height: HEIGHT,
                    },
                    Pos { x: 1, y: 0 },
                    0xffb58863,
                )),
                Pos { x: 1, y: 0 },
            ), // Box::new(FallingSand::new_num_sands(100, WIDTH, HEIGHT)),
        ],
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // for _ in 0..(5 * FPS) {
        screen.render();
        window.update_with_buffer(&screen, WIDTH, HEIGHT).unwrap();
        for _ in 0..10 {
            screen.tick();
        }
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
