use crate::screen::{Channel, Component, Pixel, Pos, RgbaChannel};

#[derive(Debug, Clone, Copy)]
pub struct Blinker {
    pos: Pos,
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
            pos: self.pos,
            color,
        };

        vec![pixel]
    }

    fn tick(&mut self) {
        self.toggle();
    }
}

impl Blinker {
    pub fn new(pos: Pos) -> Self {
        Self { pos, active: false }
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
enum Direction {
    Up,
    Down,
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
}

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down,
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
    pub fn new(pixel: Pixel, channel: Channel) -> Self {
        Self {
            pixel,
            direction: Direction::Up,
            channel,
        }
    }
}
