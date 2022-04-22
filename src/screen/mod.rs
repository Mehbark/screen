use std::{collections::HashMap, fmt::Debug};
mod pixel;
pub use pixel::{Channel, Pixel, RgbaChannel};
mod pos;
pub use pos::Pos;
mod bounds;
pub use bounds::Bounds;

pub trait Component: Debug {
    fn render(&self) -> Vec<Pixel> {
        Vec::new()
    }

    fn tick(&mut self) {}
}

#[derive(Debug)]
pub struct Screen {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    children: Vec<(Box<dyn Component>, Pos)>,
}

impl std::ops::Deref for Screen {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
impl std::ops::DerefMut for Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
impl std::convert::AsRef<[u32]> for Screen {
    fn as_ref(&self) -> &[u32] {
        &self.buffer
    }
}

impl Screen {
    pub fn new(
        buffer: Vec<u32>,
        width: usize,
        height: usize,
        children: Vec<(Box<dyn Component>, Pos)>,
    ) -> Self {
        Self {
            buffer,
            width,
            height,
            children,
        }
    }

    #[must_use]
    pub fn get(&self, Pos { x, y }: Pos) -> Option<&u32> {
        self.buffer.get(y * self.width + x)
    }

    #[must_use]
    pub fn get_mut(&mut self, Pos { x, y }: Pos) -> Option<&mut u32> {
        self.buffer.get_mut(y * self.width + x)
    }

    pub fn set(&mut self, pos: Pos, new: u32) -> bool {
        if let Some(pixel) = self.get_mut(pos) {
            *pixel = new;
            true
        } else {
            false
        }
    }

    pub fn set_pixel(&mut self, Pixel { pos, color }: Pixel) -> bool {
        self.set(pos, color)
    }
}

impl Screen {
    pub fn render(&mut self) {
        let mut new_pixels = HashMap::new();

        for child in &mut self.children {
            for pixel in child.0.render() {
                new_pixels.entry(child.1).or_insert(Vec::new()).push(pixel);
            }
            // println!("{child:#?}");
        }

        for (pos, pixels) in new_pixels {
            for pixel in pixels {
                let in_bounds = self.set_pixel(Pixel {
                    color: pixel.color,
                    pos: pixel.pos + pos,
                });
                // assert!(in_bounds);
            }
        }
    }

    pub fn tick(&mut self) {
        for child in &mut self.children {
            child.0.tick();
        }
    }
}
