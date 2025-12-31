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

    pub fn position(&self) -> u16 {
        self.position
    }

    fn left(&mut self, rotation: u16) {
        let rotation = rotation % 100;
        if rotation > self.position {
            self.position = 100 - (rotation - self.position);
        } else {
            self.position = self.position - rotation
        }
    }

    fn right(&mut self, rotation: u16) {
        let rotation = rotation % 100;
        if rotation + self.position > 99 {
            self.position = rotation - (100 - self.position);
        } else {
            self.position = self.position + rotation;
        }
    }

    pub fn rotate(&mut self, kind: RotationKind, rotation: u16) {
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

    #[test]
    fn rotate() {
        let mut dial = Dial { position: 0 };
        dial.rotate(RotationKind::Right, 80);
        assert_eq!(dial.position, 80);
        dial.rotate(RotationKind::Left, 20);
        assert_eq!(dial.position, 60);
    }
}
