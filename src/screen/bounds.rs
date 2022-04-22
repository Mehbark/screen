use crate::components::Direction;

use super::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds {
    pub width: usize,
    pub height: usize,
}

impl Bounds {
    /// Returns a tuple with the first field being horizontal edge (if any) and the second being the vertical edge (if any)
    pub fn edges(&self, &Pos { x, y }: &Pos) -> (Option<Direction>, Option<Direction>) {
        let mut on = (None, None);

        if x == 0 {
            on.0 = Some(Direction::Left);
        }
        if x == self.width - 1 {
            on.0 = Some(Direction::Right);
        }

        if y == 0 {
            on.1 = Some(Direction::Up);
        }
        if y == self.height - 1 {
            on.1 = Some(Direction::Down);
        }

        on
    }
}
