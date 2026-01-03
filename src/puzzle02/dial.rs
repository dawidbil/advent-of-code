use std::fmt;

#[derive(Debug)]
pub struct Dial {
    position: u16,
}

#[derive(Debug, PartialEq)]
pub enum RotationKind {
    Left,
    Right,
}

impl Dial {
    pub fn new(position: u16) -> Self {
        Self { position }
    }

    /// Rotates the dial towards the lower positions
    /// Returns the amount of times the dial steps on zero position
    fn left(&mut self, rotation: u16) -> u16 {
        let zero_hits = self.left_zero_hits(rotation);
        let rotation_modulo = rotation % 100;
        if rotation_modulo > self.position {
            self.position = 100 - (rotation_modulo - self.position);
        } else {
            self.position = self.position - rotation_modulo;
        }
        zero_hits
    }

    /// Returns the amount of times the dial steps on zero position
    fn left_zero_hits(&self, rotation: u16) -> u16 {
        if rotation == 0 {
            return 0
        }
        let revolutions = rotation / 100;
        let offset = rotation % 100;
        if offset >= self.position {
            if self.position == 0 {
                revolutions
            } else {
                revolutions + 1
            }
        } else {
            revolutions
        }
    }

    /// Rotates the dial towards the upper positions
    /// Returns the amount of times the dial steps on zero position
    fn right(&mut self, rotation: u16) -> u16 {
        let zero_hits = self.right_zero_hits(rotation);
        let rotation_modulo = rotation % 100;
        if rotation_modulo + self.position > 99 {
            self.position = rotation_modulo - (100 - self.position);
        } else {
            self.position = self.position + rotation_modulo;
        }
        zero_hits
    }

    /// Returns the amount of times the dial steps on zero position
    fn right_zero_hits(&self, rotation: u16) -> u16 {
        if rotation == 0 {
            return 0
        }
        let revolutions = rotation / 100;
        let offset = rotation % 100;
        if self.position + offset >= 100 {
            if self.position == 0 {
                revolutions
            } else {
                revolutions + 1
            }
        } else {
            revolutions
        }
    }

    /// Rotates the dial in left or right direction
    /// Returns the amount of times the dial steps on zero position
    pub fn rotate(&mut self, kind: RotationKind, rotation: u16) -> u16 {
        match kind {
            RotationKind::Left => self.left(rotation),
            RotationKind::Right => self.right(rotation),
        }
    }
}

impl fmt::Display for Dial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dial is positioned at {}", self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left() {
        let mut dial = Dial { position: 0 };
        dial.left(1);
        assert_eq!(dial.position, 99);
        dial.left(99);
        assert_eq!(dial.position, 0);
        dial.left(100);
        assert_eq!(dial.position, 0);
        dial.left(50);
        assert_eq!(dial.position, 50);
    }

    fn assert_left_zero_hits(position: u16, rotation: u16, expected: u16) {
        let dial = Dial { position };
        let hits = dial.left_zero_hits(rotation);
        assert_eq!(hits, expected);
    }

    #[test]
    fn left_zero_hits() {
        assert_left_zero_hits(0, 0, 0);
        assert_left_zero_hits(0, 1, 0);
        assert_left_zero_hits(0, 100, 1);
        assert_left_zero_hits(0, 101, 1);
        assert_left_zero_hits(0, 200, 2);
        assert_left_zero_hits(1, 1, 1);
        assert_left_zero_hits(1, 2, 1);
        assert_left_zero_hits(1, 100, 1);
        assert_left_zero_hits(1, 101, 2);
        assert_left_zero_hits(1, 102, 2);
        assert_left_zero_hits(1, 200, 2);
        assert_left_zero_hits(2, 1, 0);
    }

    #[test]
    fn right() {
        let mut dial = Dial { position: 0 };
        dial.right(1);
        assert_eq!(dial.position, 1);
        dial.right(99);
        assert_eq!(dial.position, 0);
        dial.right(100);
        assert_eq!(dial.position, 0);
        dial.right(50);
        assert_eq!(dial.position, 50);
    }

    fn assert_right_zero_hits(position: u16, rotation: u16, expected: u16) {
        let dial = Dial { position };
        let hits = dial.right_zero_hits(rotation);
        assert_eq!(hits, expected);
    }

    #[test]
    fn right_zero_hits() {
        assert_right_zero_hits(0, 0, 0);
        assert_right_zero_hits(0, 1, 0);
        assert_right_zero_hits(0, 100, 1);
        assert_right_zero_hits(0, 101, 1);
        assert_right_zero_hits(0, 200, 2);
        assert_right_zero_hits(99, 1, 1);
        assert_right_zero_hits(99, 2, 1);
        assert_right_zero_hits(99, 100, 1);
        assert_right_zero_hits(99, 101, 2);
        assert_right_zero_hits(99, 102, 2);
        assert_right_zero_hits(99, 200, 2);
        assert_right_zero_hits(98, 1, 0);
    }

    #[test]
    fn rotate() {
        let mut dial = Dial { position: 0 };
        dial.rotate(RotationKind::Right, 80);
        assert_eq!(dial.position, 80);
        dial.rotate(RotationKind::Left, 20);
        assert_eq!(dial.position, 60);
    }
}
