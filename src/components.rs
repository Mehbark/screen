use itertools::Itertools;
use rand::prelude::SliceRandom;

use crate::screen::{Bounds, Channel, Component, Pixel, Pos, RgbaChannel};

#[derive(Debug, Clone, Copy)]
pub struct Blinker {
    active: bool,
}

impl Component for Blinker {
    fn render(&self) -> Vec<Pixel> {
        let color = if self.is_active() {
            0xffffffff
        } else {
            0x00000000
        };
        let pixel = Pixel {
            pos: Pos::default(),
            color,
        };

        vec![pixel]
    }

    fn tick(&mut self) {
        self.toggle();
    }
}

impl Blinker {
    pub fn new() -> Self {
        Self { active: false }
    }

    fn toggle(&mut self) {
        self.active = !self.active;
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Breather {
    pixel: Pixel,
    channel: Channel,
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns `true` if the direction is [`Up`].
    ///
    /// [`Up`]: Direction::Up
    #[must_use]
    fn is_up(&self) -> bool {
        matches!(self, Self::Up)
    }

    /// Returns `true` if the direction is [`Down`].
    ///
    /// [`Down`]: Direction::Down
    #[must_use]
    fn is_down(&self) -> bool {
        matches!(self, Self::Down)
    }

    /// Returns `true` if the direction is [`Left`].
    ///
    /// [`Left`]: Direction::Left
    #[must_use]
    fn is_left(&self) -> bool {
        matches!(self, Self::Left)
    }

    /// Returns `true` if the direction is [`Right`].
    ///
    /// [`Right`]: Direction::Right
    #[must_use]
    fn is_right(&self) -> bool {
        matches!(self, Self::Right)
    }
}

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Component for Breather {
    fn render(&self) -> Vec<Pixel> {
        vec![self.pixel]
    }

    fn tick(&mut self) {
        self.pixel = {
            self.pixel.map_channel(self.channel, |c| {
                let new = if self.direction.is_up() {
                    c.saturating_add(1)
                } else {
                    c.saturating_sub(1)
                };

                if new == 255 || new == 0 {
                    self.direction = !self.direction;
                }
                // println!("{new}");
                new
            })
        };
        // println!("{:#08x}", self.pixel.color);
    }
}

impl Breather {
    pub fn new(color: u32, channel: Channel) -> Self {
        Self {
            pixel: Pixel {
                pos: Pos { x: 0, y: 0 },
                color,
            },
            direction: Direction::Up,
            channel,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FallingSand {
    sands: Vec<Pos>,
    width: usize,
    height: usize,
}

impl FallingSand {
    pub fn new(sands: Vec<Pos>, width: usize, height: usize) -> Self {
        Self {
            sands,
            width,
            height,
        }
    }
    pub fn new_num_sands(num_sands: usize, width: usize, height: usize) -> Self {
        Self {
            sands: (0..width)
                .cycle()
                .take(num_sands)
                .map(|x| Pos { x, y: 0 })
                .collect(),
            width,
            height,
        }
    }
}

impl Component for FallingSand {
    fn render(&self) -> Vec<Pixel> {
        self.sands
            .iter()
            .map(|sand| Pixel {
                pos: *sand,
                color: 0xcdd647,
            })
            .collect()
    }

    fn tick(&mut self) {
        let mut new_sands = Vec::new();

        self.sands.shuffle(&mut rand::thread_rng());

        for sand in &self.sands {
            new_sands.push(self.sand_move(sand));
        }

        std::mem::swap(&mut new_sands, &mut self.sands);
    }
}

impl FallingSand {
    #[must_use]
    fn is_available(&self, pos: Pos) -> bool {
        self.is_in_bounds(pos) && !self.sands.iter().any(|&sand| sand == pos)
    }

    #[must_use]
    fn is_in_bounds(&self, Pos { x, y }: Pos) -> bool {
        x < self.width && y < self.height
    }

    fn sand_move(&self, sand: &Pos) -> Pos {
        [
            *sand + Pos { x: 0, y: 1 },
            *sand + Pos { x: 0, y: 1 } - Pos { x: 1, y: 0 },
            *sand + Pos { x: 1, y: 1 },
            *sand + Pos { x: 1, y: 0 },
            *sand - Pos { x: 1, y: 0 },
        ]
        .into_iter()
        .find(|&pos| self.is_available(pos))
        .unwrap_or(*sand)
    }
}

#[derive(Debug)]
pub struct Laser {
    bounds: Bounds,
    body: Vec<Pos>,
    directions: (Direction, Direction),
    step: usize,
    offset: Pos,
    color: u32,
    color_direction: Direction,
}

impl Laser {
    pub fn new(bounds: Bounds, offset: Pos, color: u32) -> Self {
        Self {
            bounds,
            body: vec![Pos { x: 0, y: 0 }],
            directions: (Direction::Left, Direction::Up),
            step: 0,
            offset,
            color,
            color_direction: Direction::Up,
        }
    }
}

impl Component for Laser {
    fn render(&self) -> Vec<Pixel> {
        self.body
            .iter()
            .map(|&pos| Pixel {
                pos,
                color: self.color,
            })
            .collect()
    }

    fn tick(&mut self) {
        let head = *self.body.last().expect("somehow no body on laser lol");
        // println!("{head:#?}");
        // println!("{:#?}", self.directions);

        let (horizontal_edge, vertical_edge) = self.bounds.edges(&head);
        if horizontal_edge.is_some() {
            self.directions.0 = !self.directions.0;
        }
        if vertical_edge.is_some() {
            self.directions.1 = !self.directions.1;
        }

        let new = head + self.directions.0 + self.directions.1;
        self.body.push(new);
        if self.body.len() > 10 {
            self.body.remove(0);
        }

        // self.step = self.step.saturating_add(1);
        // if self.step % 10 == 0 {
        //     if self.color_direction.is_up() {
        //         self.color = self.color.saturating_add(1);
        //     } else {
        //         self.color = self.color.saturating_sub(1);
        //     }

        //     if self.color == 255 || self.color == 0 {
        //         self.color_direction = !self.color_direction;
        //     }
        // }
    }
}
