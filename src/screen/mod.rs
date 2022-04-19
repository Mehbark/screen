use std::fmt::Debug;
mod pixel;
pub use pixel::{Channel, Pixel, RgbaChannel};

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

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
    children: Vec<Box<dyn Component>>,
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
        children: Vec<Box<dyn Component>>,
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
        let mut new_pixels = Vec::new();

        for child in &mut self.children {
            for pixel in child.render() {
                new_pixels.push(pixel);
            }
            // println!("{child:#?}");
        }

        for pixel in new_pixels {
            let in_bounds = self.set_pixel(pixel);
            assert!(in_bounds);
        }
    }

    pub fn tick(&mut self) {
        for child in &mut self.children {
            child.tick();
        }
    }
}
