use super::{Component, Pos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pixel {
    pub pos: Pos,
    pub color: u32,
}

impl Component for Pixel {
    fn render(&self) -> Vec<Pixel> {
        vec![*self]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Alpha,
    Red,
    Green,
    Blue,
}

#[rustfmt::skip]
impl Channel {
    fn negative_mask(self) -> u32 {
        match self {
            Channel::Alpha => 0x00_ff_ff_ff,
            Channel::Red   => 0xff_00_ff_ff,
            Channel::Green => 0xff_ff_00_ff,
            Channel::Blue  => 0xff_ff_ff_00,
        }
    }

    fn positive_mask(self) -> u32 {
        match self {
            Channel::Alpha => 0xff_00_00_00,
            Channel::Red   => 0x00_ff_00_00,
            Channel::Green => 0x00_00_ff_00,
            Channel::Blue  => 0x00_00_00_ff,
        }
    }

    fn shift(self) -> u32 {
        match self {
            Channel::Alpha => 24,
            Channel::Red   => 16,
            Channel::Green => 8,
            Channel::Blue  => 0,
        }
    }
}

pub trait RgbaChannel: Sized + Copy {
    fn set_channel(self, channel: Channel, new: u8) -> Self;
    fn get_channel(self, channel: Channel) -> u8;
    fn map_channel<F>(self, channel: Channel, mut op: F) -> Self
    where
        F: FnMut(u8) -> u8,
    {
        let res = op(self.get_channel(channel));
        self.set_channel(channel, res)
    }
}

impl RgbaChannel for u32 {
    fn set_channel(self, channel: Channel, new: u8) -> u32 {
        let vulnerable_in_a_convenient_channel = self & channel.negative_mask();
        let shifted = (new as u32) << channel.shift();

        vulnerable_in_a_convenient_channel | shifted
    }

    fn get_channel(self, channel: Channel) -> u8 {
        let isolated = self & channel.positive_mask();
        let shifted = isolated >> channel.shift();

        shifted as u8
    }
}

#[test]
fn chanel_lol() {
    let cool: u32 = 0x00_00_00_00;

    let set_cool = cool.set_channel(Channel::Red, 0xff);
    assert_eq!(set_cool, 0x00_ff_00_00);

    let get_cool = set_cool.get_channel(Channel::Red);
    assert_eq!(get_cool, 0xff);

    let map_cool = set_cool.map_channel(Channel::Red, |c| c - 1);
    assert_eq!(map_cool, 0x00_fe_00_00);
}

impl RgbaChannel for Pixel {
    fn set_channel(self, channel: Channel, new: u8) -> Self {
        Self {
            pos: self.pos,
            color: self.color.set_channel(channel, new),
        }
    }

    fn get_channel(self, channel: Channel) -> u8 {
        self.color.get_channel(channel)
    }
}
